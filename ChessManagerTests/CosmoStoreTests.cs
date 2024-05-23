using System.Data;
using ChessManager.CosmoStore;
using Microsoft.Data.Sqlite;
using Dapper;

namespace ChessManagerTests;

public record Payload(string Value);


public class CosmoStoreTests : IDisposable
{

    private readonly EventStoreFactory _eventStoreFactory = new();


    [Fact]
    public async void EventStoreExists()
    {
        //Arrange
        using var dummyEventStore = await _eventStoreFactory.GetFreshEventStore<Payload>();
        //Assert
        Assert.NotNull(dummyEventStore);
    }

    [Fact]
    public async void AppendEvent()
    {
        //Arrange
        using var dummyEventStore = await _eventStoreFactory.GetFreshEventStore<Payload>();
        var streamId = new StreamId("streamId");
        var eventWrite = new EventWrite<Payload>("id", "correlationId", "causationId", "name", new Payload("data"));
        //Act
        var eventRead = await dummyEventStore.AppendEvent(streamId, eventWrite);
        //Assert
        Assert.NotNull(eventRead);

    }

    [Fact]
    public async void Append100Events()
    {
        //Arrange
        using var dummyEventStore = await _eventStoreFactory.GetFreshEventStore<Payload>();
        var streamId = new StreamId("streamId");
        var eventWrites = new List<EventWrite<Payload>>();
        for (var i = 0; i < 100; i++)
        {
            eventWrites.Add(new EventWrite<Payload>($"id{i}", "correlationId", "causationId", "name", new Payload($"data{i}")));
        }
        //Act
        var eventReads = await dummyEventStore.AppendEvents(streamId, eventWrites);
        //Assert
        Assert.NotNull(eventReads);
        Assert.Equal(100, eventReads.Count);
    }

    [Fact]
    public async void GetSingleEvent()
    {
        //Arrange
        using var dummyEventStore = await _eventStoreFactory.GetFreshEventStore<Payload>();
        var streamId = new StreamId("streamId");
        var eventWrite = new EventWrite<Payload>("id", "correlationId", "causationId", "name", new Payload("data"));
        var eventRead = await dummyEventStore.AppendEvent(streamId, eventWrite);
        Assert.NotNull(eventRead);
        //Act
        var retrievedEvent = await dummyEventStore.GetEvent(streamId, 1);

        //Assert
        Assert.NotNull(retrievedEvent);
        Assert.Equal(eventRead.Id, retrievedEvent.Id);
    }

    [Fact]
    public async void GetAllEvents()
    {
        using var dummyEventStore = await _eventStoreFactory.GetFreshEventStore<Payload>();
        var streamId = new StreamId("streamId");
        var eventWrites = new List<EventWrite<Payload>>();
        for (var i = 0; i < 10; i++)
        {
            eventWrites.Add(new EventWrite<Payload>($"id{i}", "correlationId", "causationId", "name", new Payload($"data{i}")));
        }
        var eventReads = await dummyEventStore.AppendEvents(streamId, eventWrites);
        Assert.NotNull(eventReads);
        var events = await dummyEventStore.GetEvents(streamId, new AllEvents());
        Assert.NotNull(events);
        Assert.Equal(10, events.Count);
    }

    [Fact]
    public async void GetEventsFromVersion()
    {
        using var dummyEventStore = await _eventStoreFactory.GetFreshEventStore<Payload>();
        var streamId = new StreamId("streamId");
        var eventWrites = new List<EventWrite<Payload>>();
        for (var i = 0; i < 10; i++)
        {
            eventWrites.Add(new EventWrite<Payload>($"id{i}", "correlationId", "causationId", "name", new Payload($"data{i}")));
        }
        var eventReads = await dummyEventStore.AppendEvents(streamId, eventWrites);
        Assert.NotNull(eventReads);
        var events = await dummyEventStore.GetEvents(streamId, new FromVersion(5));
        Assert.NotNull(events);
        Assert.Equal(6, events.Count);
    }

    [Fact]
    public async void GetEventsToVersion()
    {
        using var dummyEventStore = await _eventStoreFactory.GetFreshEventStore<Payload>();
        var streamId = new StreamId("streamId");
        var eventWrites = new List<EventWrite<Payload>>();
        for (var i = 0; i < 10; i++)
        {
            eventWrites.Add(new EventWrite<Payload>($"id{i}", "correlationId", "causationId", "name", new Payload($"data{i}")));
        }
        var eventReads = await dummyEventStore.AppendEvents(streamId, eventWrites);
        Assert.NotNull(eventReads);
        var events = await dummyEventStore.GetEvents(streamId, new ToVersion(5));
        Assert.NotNull(events);
        Assert.Equal(5, events.Count);
    }

    [Fact]
    public async void GetEventsVersionRange()
    {
        using var dummyEventStore = await _eventStoreFactory.GetFreshEventStore<Payload>();
        var streamId = new StreamId("streamId");
        var eventWrites = new List<EventWrite<Payload>>();
        for (var i = 0; i < 10; i++)
        {
            eventWrites.Add(new EventWrite<Payload>($"id{i}", "correlationId", "causationId", "name", new Payload($"data{i}")));
        }
        var eventReads = await dummyEventStore.AppendEvents(streamId, eventWrites);
        Assert.NotNull(eventReads);
        var events = await dummyEventStore.GetEvents(streamId, new VersionRange(3, 7));
        Assert.NotNull(events);
        Assert.Equal(5, events.Count);
    }

    [Fact]
    public async void GetStreamById()
    {
        using var dummyEventStore = await _eventStoreFactory.GetFreshEventStore<Payload>();
        var streamId = new StreamId("streamId");
        var eventWrites = new List<EventWrite<Payload>>();
        for (var i = 0; i < 10; i++)
        {
            eventWrites.Add(new EventWrite<Payload>($"id{i}", "correlationId", "causationId", "name", new Payload($"data{i}")));
        }
        var eventReads = await dummyEventStore.AppendEvents(streamId, eventWrites);
        Assert.NotNull(eventReads);
        var stream = await dummyEventStore.GetStream(streamId);
        Assert.NotNull(stream);
        Assert.Equal(streamId, stream.Id);
    }

    public void Dispose()
    {
        _eventStoreFactory.Dispose();
    }
}