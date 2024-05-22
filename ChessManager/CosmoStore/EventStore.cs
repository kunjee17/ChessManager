using System;
using System.Collections.Generic;
using System.Data;
using System.Reactive.Linq;
using System.Reactive.Subjects;
using System.Threading.Tasks;
using Microsoft.Data.Sqlite;

namespace ChessManager.CosmoStore;

public class EventStore<TPayload, TVersion> : IEventStore<TPayload,TVersion>, IDisposable
{
    public const string EventCollection = "events";
    public const string StreamCollection = "streams";
    private readonly SqliteConnection _connection;
    private readonly Subject<EventRead<TPayload, TVersion>> _eventAppendedSubject = new Subject<EventRead<TPayload, TVersion>>();


    private async Task SetUpEventStoreAsync()
    {
        Console.WriteLine("Setting up EventStore...");
        await _connection.OpenAsync();
        Console.WriteLine("Connection is open");
        // Create Database for streams and events if not exist
        await _connection.CloseAsync();
        Console.WriteLine("Connection is closed");
    }

    private EventStore(SqliteConnection connection)
    {
        _connection = connection;
        EventAppended = _eventAppendedSubject.AsObservable();
        // _eventAppendedSubject.OnNext(eventRead);
    }
    
    public static async Task<EventStore<TPayload, TVersion>> CreateAsync(SqliteConnection connection)
    {
        var eventStore = new EventStore<TPayload, TVersion>(connection);
        await eventStore.SetUpEventStoreAsync();
        return eventStore;
    }

    public Task<EventRead<TPayload, TVersion>> AppendEvent(StreamId streamId, ExpectedVersion<TVersion> expectedVersion, EventWrite<TPayload> eventWrite)
    {
        throw new NotImplementedException();
    }

    public Task<List<EventRead<TPayload, TVersion>>> AppendEvents(StreamId streamId, ExpectedVersion<TVersion> expectedVersion, List<EventWrite<TPayload>> eventWrites)
    {
        throw new NotImplementedException();
    }

    public Task<EventRead<TPayload, TVersion>> GetEvent(StreamId streamId, TVersion version)
    {
        throw new NotImplementedException();
    }

    public Task<List<EventRead<TPayload, TVersion>>> GetEvents(StreamId streamId, EventsReadRange<TVersion> readRange)
    {
        throw new NotImplementedException();
    }

    public Task<List<EventRead<TPayload, TVersion>>> GetEventsByCorrelationId(Guid correlationId)
    {
        throw new NotImplementedException();
    }

    public Task<List<Stream<TVersion>>> GetStreams(StreamsReadFilter readFilter)
    {
        throw new NotImplementedException();
    }

    public Task<Stream<TVersion>> GetStream(StreamId streamId)
    {
        throw new NotImplementedException();
    }

    public IObservable<EventRead<TPayload, TVersion>> EventAppended { get; }

    public void Dispose()
    {
        _connection.Dispose();
        _eventAppendedSubject.Dispose();
        GC.SuppressFinalize(this);
    }
}