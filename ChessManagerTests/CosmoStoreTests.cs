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
        var eventRead = dummyEventStore.AppendEvent(streamId, eventWrite).Result;
        //Assert
        Assert.NotNull(eventRead);

    }

    [Fact]
    public void Append100Events()
    {
        //Arrange
        using var dummyEventStore = _eventStoreFactory.GetFreshEventStore<Payload>().Result;
        var streamId = new StreamId("streamId");
        var eventWrites = new List<EventWrite<Payload>>();
        for (var i = 0; i < 100; i++)
        {
            eventWrites.Add(new EventWrite<Payload>($"id{i}", "correlationId", "causationId", "name", new Payload($"data{i}")));
        }
        //Act
        var eventReads = dummyEventStore.AppendEvents(streamId, eventWrites).Result;
        //Assert
        Assert.NotNull(eventReads);
        Assert.Equal(100, eventReads.Count);
    }

    [Fact]
    public void GetSingleEvent()
    {
        //Arrange
        using var dummyEventStore = _eventStoreFactory.GetFreshEventStore<Payload>().Result;
        var streamId = new StreamId("streamId");
        var eventWrite = new EventWrite<Payload>("id", "correlationId", "causationId", "name", new Payload("data"));
        var eventRead = dummyEventStore.AppendEvent(streamId, eventWrite).Result;
        Assert.NotNull(eventRead);
        //Act
        var retrievedEvent = dummyEventStore.GetEvent(streamId, 1).Result;

        //Assert
        Assert.NotNull(retrievedEvent);
        Assert.Equal(eventRead.Id, retrievedEvent.Id);
    }

    [Fact]
    public void GetAllEvents()
    {
        using var dummyEventStore = _eventStoreFactory.GetFreshEventStore<Payload>().Result;
        var streamId = new StreamId("streamId");
        var eventWrites = new List<EventWrite<Payload>>();
        for (var i = 0; i < 10; i++)
        {
            eventWrites.Add(new EventWrite<Payload>($"id{i}", "correlationId", "causationId", "name", new Payload($"data{i}")));
        }
        var eventReads = dummyEventStore.AppendEvents(streamId, eventWrites).Result;
        Assert.NotNull(eventReads);
        var events = dummyEventStore.GetEvents(streamId, new AllEvents()).Result;
        Assert.NotNull(events);
        Assert.Equal(10, events.Count);
    }

    [Fact]
    public void GetEventsFromVersion()
    {
        using var dummyEventStore = _eventStoreFactory.GetFreshEventStore<Payload>().Result;
        var streamId = new StreamId("streamId");
        var eventWrites = new List<EventWrite<Payload>>();
        for (var i = 0; i < 10; i++)
        {
            eventWrites.Add(new EventWrite<Payload>($"id{i}", "correlationId", "causationId", "name", new Payload($"data{i}")));
        }
        var eventReads = dummyEventStore.AppendEvents(streamId, eventWrites).Result;
        Assert.NotNull(eventReads);
        var events = dummyEventStore.GetEvents(streamId, new FromVersion(5)).Result;
        Assert.NotNull(events);
        Assert.Equal(6, events.Count);
    }

    [Fact]
    public void GetEventsToVersion()
    {
        using var dummyEventStore = _eventStoreFactory.GetFreshEventStore<Payload>().Result;
        var streamId = new StreamId("streamId");
        var eventWrites = new List<EventWrite<Payload>>();
        for (var i = 0; i < 10; i++)
        {
            eventWrites.Add(new EventWrite<Payload>($"id{i}", "correlationId", "causationId", "name", new Payload($"data{i}")));
        }
        var eventReads = dummyEventStore.AppendEvents(streamId, eventWrites).Result;
        Assert.NotNull(eventReads);
        var events = dummyEventStore.GetEvents(streamId, new ToVersion(5)).Result;
        Assert.NotNull(events);
        Assert.Equal(5, events.Count);
    }

    [Fact]
    public void GetEventsVersionRange()
    {
        using var dummyEventStore = _eventStoreFactory.GetFreshEventStore<Payload>().Result;
        var streamId = new StreamId("streamId");
        var eventWrites = new List<EventWrite<Payload>>();
        for (var i = 0; i < 10; i++)
        {
            eventWrites.Add(new EventWrite<Payload>($"id{i}", "correlationId", "causationId", "name", new Payload($"data{i}")));
        }
        var eventReads = dummyEventStore.AppendEvents(streamId, eventWrites).Result;
        Assert.NotNull(eventReads);
        var events = dummyEventStore.GetEvents(streamId, new VersionRange(3, 7)).Result;
        Assert.NotNull(events);
        Assert.Equal(5, events.Count);
    }

    [Fact]
    public void GetStreamById()
    {
        using var dummyEventStore = _eventStoreFactory.GetFreshEventStore<Payload>().Result;
        var streamId = new StreamId("streamId");
        var eventWrites = new List<EventWrite<Payload>>();
        for (var i = 0; i < 10; i++)
        {
            eventWrites.Add(new EventWrite<Payload>($"id{i}", "correlationId", "causationId", "name", new Payload($"data{i}")));
        }
        var eventReads = dummyEventStore.AppendEvents(streamId, eventWrites).Result;
        Assert.NotNull(eventReads);
        var stream = dummyEventStore.GetStream(streamId).Result;
        Assert.NotNull(stream);
        Assert.Equal(streamId, stream.Id);
    }

    public void Dispose()
    {
        _eventStoreFactory.Dispose();
    }
}