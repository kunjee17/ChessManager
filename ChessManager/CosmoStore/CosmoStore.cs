using System;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace ChessManager.CosmoStore
{
    public record StreamId(string Value);

    public abstract record ExpectedVersion<TVersion>();
    public record Any<TVersion> : ExpectedVersion<TVersion>;
    public record NoStream<TVersion> : ExpectedVersion<TVersion>;
    public record Exact<TVersion>(TVersion Version) : ExpectedVersion<TVersion>;

    public abstract record EventsReadRange<TVersion>();
    public record AllEvents<TVersion> : EventsReadRange<TVersion>;
    public record FromVersion<TVersion>(TVersion Version) : EventsReadRange<TVersion>;
    public record ToVersion<TVersion>(TVersion Version) : EventsReadRange<TVersion>;
    public record VersionRange<TVersion>(TVersion FromVersion, TVersion ToVersion) : EventsReadRange<TVersion>;

    public abstract record StreamsReadFilter();
    public record AllStreams() : StreamsReadFilter;
    public record StartsWith(string Value) : StreamsReadFilter;
    public record EndsWith(string Value) : StreamsReadFilter;
    public record Contains(string Value) : StreamsReadFilter;

    public record EventRead<TPayload, TVersion>(
        Guid Id,
        Guid? CorrelationId,
        Guid? CausationId,
        StreamId StreamId,
        TVersion Version,
        string Name,
        TPayload Data,
        DateTime CreatedUtc
    );

    public record EventWrite<TPayload>(
        Guid Id,
        Guid? CorrelationId,
        Guid? CausationId,
        string Name,
        TPayload Data,
        TPayload? Metadata
    );

    public record Stream<TVersion>(
        StreamId Id,
        TVersion LastVersion,
        DateTime LastUpdatedUtc
    );

    public interface IEventStore<TPayload, TVersion>
    {
        Task<EventRead<TPayload, TVersion>> AppendEvent(StreamId streamId, ExpectedVersion<TVersion> expectedVersion, EventWrite<TPayload> eventWrite);
        Task<List<EventRead<TPayload, TVersion>>> AppendEvents(StreamId streamId, ExpectedVersion<TVersion> expectedVersion, List<EventWrite<TPayload>> eventWrites);
        Task<EventRead<TPayload, TVersion>> GetEvent(StreamId streamId, TVersion version);
        Task<List<EventRead<TPayload, TVersion>>> GetEvents(StreamId streamId, EventsReadRange<TVersion> readRange);
        Task<List<EventRead<TPayload, TVersion>>> GetEventsByCorrelationId(Guid correlationId);
        Task<List<Stream<TVersion>>> GetStreams(StreamsReadFilter readFilter);
        Task<Stream<TVersion>> GetStream(StreamId streamId);
        IObservable<EventRead<TPayload, TVersion>> EventAppended { get; }
    }
}