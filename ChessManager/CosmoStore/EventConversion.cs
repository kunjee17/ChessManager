using System;

namespace ChessManager.CosmoStore;

public static class EventConversion
{
    public static EventRead<TPayload, TVersion> EventWriteToEventRead<TPayload, TVersion>(
        StreamId streamId,
        TVersion version,
        DateTime createdUtc,
        EventWrite<TPayload> x)
    {
        return new EventRead<TPayload, TVersion>(
            x.Id,
            x.CorrelationId,
            x.CausationId,
            streamId,
            version,
            x.Name,
            x.Data,
            createdUtc
        );
    }
}