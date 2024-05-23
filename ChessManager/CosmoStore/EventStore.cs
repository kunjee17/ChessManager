using System;
using System.Collections.Generic;
using System.Linq;
using System.Reactive.Linq;
using System.Reactive.Subjects;
using System.Text.Json;
using System.Threading.Tasks;
using Dapper;
using DynamicData.Kernel;
using Microsoft.Data.Sqlite;

namespace ChessManager.CosmoStore;

record DbEventStream(string Id, long LastVersion, DateTime LastUpdatedUtc);
record DbEvent(string Id, string? CorrelationId, string? CausationId, string StreamId, long Version, string Name, string Data, DateTime CreatedUtc);

public class EventStore<TPayload> : IEventStore<TPayload>, IDisposable
{
    private const string EventCollection = "events";
    private const string StreamCollection = "streams";
    private readonly SqliteConnection _connection;
    private readonly Subject<EventRead<TPayload>> _eventAppendedSubject = new Subject<EventRead<TPayload>>();

    private DbEvent CreateDbEventFromRow(dynamic x)
    {
        return new DbEvent(x.id, x.correlationId, x.causationId, x.streamId, x.version, x.name, x.data, Convert.ToDateTime(x.createdUtc));
    }

    private DbEventStream CreateDbEventStreamFromRow(dynamic x)
    {
        return new DbEventStream(x.id, x.lastVersion, Convert.ToDateTime(x.lastUpdatedUtc));
    }

    private EventWrite<TPayload> GetEventWriteFromDbEvent(DbEvent x)
    {
        return new EventWrite<TPayload>(x.Id, x.CorrelationId, x.CausationId, x.Name,
            JsonSerializer.Deserialize<TPayload>(x.Data));
    }

    private async Task SetUpEventStoreAsync()
    {
        await _connection.OpenAsync();
        const string createStreamTable = $"""

                                                      CREATE TABLE IF NOT EXISTS {StreamCollection} (
                                                          id TEXT PRIMARY KEY,
                                                          lastVersion INTEGER,
                                                          lastUpdatedUtc date default (datetime('now','utc'))
                                                      )
                                          """;
        await _connection.ExecuteAsync(createStreamTable);
        const string createEventsTable = $"""

                                                      CREATE TABLE IF NOT EXISTS {EventCollection} (
                                                          id TEXT PRIMARY KEY,
                                                          correlationId TEXT default null,
                                                          causationId TEXT default null,
                                                          streamId TEXT not null,
                                                          version INTEGER,
                                                          name VARCHAR(255) not null,
                                                          data json not null,
                                                          createdUtc date default (datetime('now','utc')),
                                                          CONSTRAINT fk_stream FOREIGN KEY (streamId) REFERENCES {StreamCollection}(id) ON DELETE CASCADE
                                                      )
                                          """;

        await _connection.ExecuteAsync(createEventsTable);
        const string createTimeStampTrigger = $"""

                                                      CREATE TRIGGER IF NOT EXISTS update_timestamp
                                                      AFTER UPDATE ON {StreamCollection}
                                                      BEGIN
                                                          UPDATE {StreamCollection}
                                                          SET lastUpdatedUtc = datetime('now','utc')
                                                          WHERE id = NEW.id;
                                                      END
                                          """;
        await _connection.ExecuteAsync(createTimeStampTrigger);
        await _connection.CloseAsync();
    }

    private EventStore(SqliteConnection connection)
    {
        _connection = connection;
        EventAppended = _eventAppendedSubject.AsObservable();
        // _eventAppendedSubject.OnNext(eventRead);
    }


    public static async Task<EventStore<TPayload>> CreateAsync(SqliteConnection connection)
    {
        var eventStore = new EventStore<TPayload>(connection);
        await eventStore.SetUpEventStoreAsync();
        return eventStore;
    }

    //For simplicity, we are using long as version type and taking that stream is only moving forward
    private async Task<List<EventRead<TPayload>>> ProcessEvents(StreamId streamId, List<EventWrite<TPayload>> eventWrites)
    {
        await _connection.OpenAsync();
        // Check if stream based on stream id exist
        var existingStream = await _connection.QuerySingleOrDefaultAsync<DbEventStream>($"SELECT * FROM {StreamCollection} WHERE id = @Id LIMIT 1", new { Id = streamId.Value });
        var lastVersion =  existingStream?.LastVersion ?? 0;
        var nextVersion = lastVersion + 1;
        var ops = eventWrites.Select((x, index) => EventConversion.EventWriteToEventRead(streamId, nextVersion + index, x));
        var updatedStream = new DbEventStream(streamId.Value, lastVersion + eventWrites.Count, DateTime.UtcNow);
        //Insert of update stream
        await using var tr  = await _connection.BeginTransactionAsync();
        await _connection.ExecuteAsync($"INSERT INTO {StreamCollection} (id, lastVersion) values (@Id, @LastVersion) ON CONFLICT (id) do update set lastVersion = @LastVersion", new {Id = updatedStream.Id, LastVersion = updatedStream.LastVersion});
        var eventReads = ops.ToList();
        foreach (var eventRead in eventReads)
        {
            await _connection.ExecuteAsync($"INSERT INTO {EventCollection} (id, correlationId, causationId, streamId, version, name, data) values (@Id, @CorrelationId, @CausationId, @StreamId, @Version, @Name, @Data)", new {Id = eventRead.Id, CorrelationId = eventRead.CorrelationId, CausationId = eventRead.CausationId, StreamId = eventRead.StreamId.Value, Version = eventRead.Version, Name = eventRead.Name, Data = JsonSerializer.Serialize(eventRead.Data)});
        }
        await tr.CommitAsync();


        return eventReads.ToList();
    }

    public Task<EventRead<TPayload>> AppendEvent(StreamId streamId, EventWrite<TPayload> eventWrite)
    {
        return AppendEvents(streamId, [eventWrite]).ContinueWith(x => x.Result.First());
    }

    public async Task<List<EventRead<TPayload>>> AppendEvents(StreamId streamId, List<EventWrite<TPayload>> eventWrites)
    {
        if (eventWrites.Count == 0)
        {
            return [];
        }

        return await ProcessEvents(streamId, eventWrites);
    }

    public Task<List<EventRead<TPayload>>> GetEvents(StreamId streamId, EventsReadRange readRange)
    {
        return Task.FromResult(readRange switch
        {
            AllEvents => _connection
                .QueryAsync($"SELECT * FROM {EventCollection} WHERE streamId = @StreamId",
                    new { StreamId = streamId.Value })
                .ContinueWith(x => x.Result.Select(CreateDbEventFromRow))
                .ContinueWith(x => x.Result.Select(x => EventConversion.EventWriteToEventRead(streamId, x.Version,
                    GetEventWriteFromDbEvent(x)))).Result
                .ToList(),
            FromVersion fromVersion => _connection
                .QueryAsync($"SELECT * FROM {EventCollection} WHERE streamId = @StreamId AND version >= @Version",
                    new { StreamId = streamId.Value, Version = fromVersion.Version })
                .ContinueWith(x => x.Result.Select(CreateDbEventFromRow))
                .ContinueWith(x => x.Result.Select(x => EventConversion.EventWriteToEventRead(streamId, x.Version,
                    GetEventWriteFromDbEvent(x)))).Result.ToList(),
            ToVersion toVersion => _connection
                .QueryAsync($"SELECT * FROM {EventCollection} WHERE streamId = @StreamId AND version <= @Version",
                    new { StreamId = streamId.Value, Version = toVersion.Version })
                .ContinueWith(x => x.Result.Select(CreateDbEventFromRow))
                .ContinueWith(x => x.Result.Select(x => EventConversion.EventWriteToEventRead(streamId, x.Version,
                    GetEventWriteFromDbEvent(x)))).Result
                .ToList(),
            VersionRange versionRange => _connection
                .QueryAsync(
                    $"SELECT * FROM {EventCollection} WHERE streamId = @StreamId AND version >= @FromVersion AND version <= @ToVersion",
                    new
                    {
                        StreamId = streamId.Value,
                        FromVersion = versionRange.FromVersion,
                        ToVersion = versionRange.ToVersion
                    })
                .ContinueWith(x => x.Result.Select(CreateDbEventFromRow))
                .ContinueWith(x => x.Result.Select(x => EventConversion.EventWriteToEventRead(streamId, x.Version,
                    GetEventWriteFromDbEvent(x)))).Result
                .ToList(),
            _ => (List<EventRead<TPayload>>) []
        });
    }

    public async Task<EventRead<TPayload>> GetEvent(StreamId streamId, long version)
    {
        var filter = new VersionRange(version, version + 1);
        var res = await GetEvents(streamId, filter).ContinueWith(x => x.Result.FirstOrDefault());
        if (res != null)
        {
            return res;
        }
        else
        {
            throw new Exception("Can't find desired event");
        }
    }

    public Task<List<EventRead<TPayload>>> GetEventsByCorrelationId(string correlationId)
    {
        return _connection
            .QueryAsync($"SELECT * FROM {EventCollection} WHERE correlationId = @CorrelationId",
                new { CorrelationId = correlationId.ToString() })
            .ContinueWith(x => x.Result.Select(CreateDbEventFromRow))
            .ContinueWith(x => x.Result.Select(x => EventConversion.EventWriteToEventRead(new StreamId(x.StreamId), x.Version,
                GetEventWriteFromDbEvent(x))).ToList());
    }

    public Task<List<EventRead<TPayload>>> GetEventsByCausationId(string causationId)
    {
        return _connection
            .QueryAsync($"SELECT * FROM {EventCollection} WHERE correlationId = @CausationId",
                new { CausationId = causationId.ToString() })
            .ContinueWith(x => x.Result.Select(CreateDbEventFromRow))
            .ContinueWith(x => x.Result.Select(x => EventConversion.EventWriteToEventRead(new StreamId(x.StreamId), x.Version,
                GetEventWriteFromDbEvent(x))).ToList());
    }

    public Task<List<Stream>> GetStreams(StreamsReadFilter readFilter)
    {
        return readFilter switch
        {
            AllStreams => _connection
                .QueryAsync($"SELECT * FROM {StreamCollection}")
                .ContinueWith(x => x.Result.Select(CreateDbEventStreamFromRow))
                .ContinueWith(x => x.Result.Select(x => new Stream(new StreamId(x.Id), x.LastVersion, x.LastUpdatedUtc)).ToList()),
            StartsWith startsWith => _connection
                .QueryAsync($"SELECT * FROM {StreamCollection} WHERE id LIKE @Value",
                    new { Value = $"{startsWith.Value}%" })
                .ContinueWith(x => x.Result.Select(CreateDbEventStreamFromRow))
                .ContinueWith(x => x.Result.Select(x => new Stream(new StreamId(x.Id), x.LastVersion, x.LastUpdatedUtc)).ToList()),
            EndsWith endsWith => _connection
                .QueryAsync($"SELECT * FROM {StreamCollection} WHERE id LIKE @Value",
                    new { Value = $"%{endsWith.Value}" })
                .ContinueWith(x => x.Result.Select(CreateDbEventStreamFromRow))
                .ContinueWith(x => x.Result.Select(x => new Stream(new StreamId(x.Id), x.LastVersion, x.LastUpdatedUtc)).ToList()),
            Contains contains => _connection
                .QueryAsync($"SELECT * FROM {StreamCollection} WHERE id LIKE @Value",
                    new { Value = $"%{contains.Value}%" })
                .ContinueWith(x => x.Result.Select(CreateDbEventStreamFromRow))
                .ContinueWith(x => x.Result.Select(x => new Stream(new StreamId(x.Id), x.LastVersion, x.LastUpdatedUtc)).ToList()),
            _ => Task.FromResult(new List<Stream>())
        };
    }

    public Task<Stream?> GetStream(StreamId streamId)
    {
        return _connection
            .QuerySingleOrDefaultAsync($"SELECT * FROM {StreamCollection} WHERE id = @Id",
                new { Id = streamId.Value })
            .ContinueWith(x => x.Result != null
                ? CreateDbEventStreamFromRow(x.Result)
                : null)
            .ContinueWith(x => x.Result != null ? new Stream(new StreamId(x.Result.Id), x.Result.LastVersion, x.Result.LastUpdatedUtc) : null);
    }

    public IObservable<EventRead<TPayload>> EventAppended { get; }

    public void Dispose()
    {
        _connection.Dispose();
        _eventAppendedSubject.Dispose();
        GC.SuppressFinalize(this);
    }
}