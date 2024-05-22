using System.Data;
using ChessManager.CosmoStore;
using Microsoft.Data.Sqlite;
using Dapper;

namespace ChessManagerTests;

public record Payload(string Value);

public class DummyEventStore : IDisposable
{
    private const string CosmosConnectionString = "cosmosStoreTest.db";
    public SqliteConnection Connection { get; }
    public EventStore<Payload> TestEventStore { get; }

    public DummyEventStore()
    {
        //Create connection for SQLite file database
        const string connectionString = $"Data Source={CosmosConnectionString};";
        Connection = new SqliteConnection(connectionString);
        TestEventStore = EventStore<Payload>.CreateAsync(Connection).Result;
    }


    public void Dispose()
    {
        Console.WriteLine("Teardown CosmoStoreTests");
        //Close any active database connection
        Connection.Close();
        //Delete the database
        if (File.Exists(CosmosConnectionString))
        {
            Console.WriteLine("Deleting database");
            File.Delete(CosmosConnectionString);
        }
        GC.SuppressFinalize(this);
    }


}


public class CosmoStoreTests
{
    [Fact]
    public void DatabaseConnectionExists()
    {
        //Arrange
        using var connection = new DummyEventStore().Connection;
        connection.Open();
        //Assert
        Assert.Equal(ConnectionState.Open, connection.State);
    }

    [Fact]
    public async void EventStoreExists()
    {
        //Arrange
        var connection = new SqliteConnection("Data Source=:memory:;Mode=Memory;Cache=Shared;");
        using var dummyEventStore = await EventStore<Payload>.CreateAsync(connection);
        //Assert
        Assert.NotNull(dummyEventStore);
    }

    [Fact]
    public void AppendEvent()
    {
        //Arrange
        using var dummyEventStore = new DummyEventStore();
        var streamId = new StreamId("streamId");
        var eventWrite = new EventWrite<Payload>("id", "correlationId", "causationId", "name", new Payload("data"));
        //Act
        var eventRead = dummyEventStore.TestEventStore.AppendEvent(streamId, eventWrite).Result;
        //Assert
        Assert.NotNull(eventRead);

    }

    // [Fact]
    // public void Append100Events()
    // {
    //     //Arrange
    //     using var dummyEventStore = new DummyEventStore();
    //     var streamId = new StreamId("streamId");
    //     var eventWrites = new List<EventWrite<Payload>>();
    //     for (var i = 0; i < 100; i++)
    //     {
    //         eventWrites.Add(new EventWrite<Payload>($"id{i}", "correlationId", "causationId", "name", new Payload($"data{i}")));
    //     }
    //     //Act
    //     var eventReads = dummyEventStore.TestEventStore.AppendEvents(streamId, eventWrites).Result;
    //     //Assert
    //     Assert.NotNull(eventReads);
    //     Assert.Equal(100, eventReads.Count);
    // }
}