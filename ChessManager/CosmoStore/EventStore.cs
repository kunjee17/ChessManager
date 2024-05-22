using System;
using System.Collections.Generic;
using System.Data;
using System.Reactive.Linq;
using System.Reactive.Subjects;
using System.Threading.Tasks;

namespace ChessManager.CosmoStore;

public class EventStore<TPayload, TVersion> : IEventStore<TPayload,TVersion>
{
    public const string EventCollection = "events";
    public const string StreamCollection = "streams";
    private readonly IDbConnection _connection;
    private readonly Subject<EventRead<TPayload, TVersion>> _eventAppendedSubject = new Subject<EventRead<TPayload, TVersion>>();


    private void setUpEventStore()
    {
        Console.WriteLine("Setting up EventStore...");
        _connection.Open();
        Console.WriteLine("Connection is open");
        // Create Database for streams and events if not exist
        _connection.Close();
        Console.WriteLine("Connection is open");
    }

    public EventStore(IDbConnection connection)
    {
        _connection = connection;
        EventAppended = _eventAppendedSubject.AsObservable();
        // _eventAppendedSubject.OnNext(eventRead);
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
}