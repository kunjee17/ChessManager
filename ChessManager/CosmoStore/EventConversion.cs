using System;
using System.Collections.Generic;

namespace ChessManager.CosmoStore;

public static class EventConversion
{
    public static EventRead<TPayload> EventWriteToEventRead<TPayload>(
        StreamId streamId,
        long version,
        EventWrite<TPayload> x)
    {
        return new EventRead<TPayload>(
            x.Id,
            x.CorrelationId,
            x.CausationId,
            streamId,
            version,
            x.Name,
            x.Data,
            DateTime.UtcNow
        );
    }


}