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
    public EventStore<Payload,long> TestEventStore { get; }
    
    public DummyEventStore()
    {
        Console.WriteLine("Setup CosmoStoreTests");
        //Create connection for SQLite file database
        const string connectionString = $"Data Source={CosmosConnectionString};";
        Connection = new SqliteConnection(connectionString);
        TestEventStore = EventStore<Payload,long>.CreateAsync(Connection).Result;
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
    public void EventStoreExists()
    {
        //Arrange
        using var dummyEventStore = new DummyEventStore().TestEventStore;
        //Assert
        Assert.NotNull(dummyEventStore);
    }
}