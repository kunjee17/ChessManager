using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace ChessManager.CosmoStore;

public interface IAggregate<TState, TCommand, TEvent>
{
    TState Init();
    TState Apply(TState state, TEvent @event); // Using '@' symbol to avoid conflicts with keywords
    List<EventWrite<TEvent>> Execute(TState state, TCommand command);
}

public static class AggregateHelper<TState, TCommand, TEvent>
{
    public static async Task<List<EventRead<TEvent>>> MakeHandler<TState, TCommand, TEvent>(
        IAggregate<TState, TCommand, TEvent> aggregate,
        IEventStore<TEvent> store,
        TCommand command,
        StreamId streamId,
        EventsReadRange range
        )
    {
        var events = await store.GetEvents(streamId, range);
        var state = events.Aggregate(aggregate.Init(), (a, b) => aggregate.Apply(a, b.Data));
        var newEvents = aggregate.Execute(state, command).ToList();
        var resEvents = await store.AppendEvents(streamId,  newEvents);
        return resEvents;
    }
}