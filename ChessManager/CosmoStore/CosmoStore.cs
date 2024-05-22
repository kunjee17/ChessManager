using System;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace ChessManager.CosmoStore
{
    public record StreamId(string Value);



    public abstract record EventsReadRange();
    public record AllEvents : EventsReadRange;
    public record FromVersion(long Version) : EventsReadRange;
    public record ToVersion(long Version) : EventsReadRange;
    public record VersionRange(long FromVersion, long ToVersion) : EventsReadRange;

    public abstract record StreamsReadFilter();
    public record AllStreams() : StreamsReadFilter;
    public record StartsWith(string Value) : StreamsReadFilter;
    public record EndsWith(string Value) : StreamsReadFilter;
    public record Contains(string Value) : StreamsReadFilter;

    public record EventRead<TPayload>(
        string Id,
        string? CorrelationId,
        string? CausationId,
        StreamId StreamId,
        long Version,
        string Name,
        TPayload Data,
        DateTime CreatedUtc
    );

    public record EventWrite<TPayload>(
        string Id,
        string? CorrelationId,
        string? CausationId,
        string Name,
        TPayload Data
    );

    public record Stream(
        StreamId Id,
        long LastVersion,
        DateTime LastUpdatedUtc
    );

    public interface IEventStore<TPayload>
    {
        Task<EventRead<TPayload>> AppendEvent(StreamId streamId, EventWrite<TPayload> eventWrite);
        Task<List<EventRead<TPayload>>> AppendEvents(StreamId streamId, List<EventWrite<TPayload>> eventWrites);
        Task<EventRead<TPayload>> GetEvent(StreamId streamId, long version);
        Task<List<EventRead<TPayload>>> GetEvents(StreamId streamId, EventsReadRange readRange);
        Task<List<EventRead<TPayload>>> GetEventsByCorrelationId(string correlationId);
        Task<List<EventRead<TPayload>>> GetEventsByCausationId(string causationId);
        Task<List<Stream>> GetStreams(StreamsReadFilter readFilter);
        Task<Stream?> GetStream(StreamId streamId);
        IObservable<EventRead<TPayload>> EventAppended { get; }
    }
}