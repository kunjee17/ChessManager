using ChessManager.CosmoStore;
using Microsoft.Data.Sqlite;

namespace ChessManagerTests;

public class DisposableDatabase : IDisposable
{
    private readonly string _databaseName;


    private DisposableDatabase(string databaseName)
    {
        _databaseName = $"Data Source={databaseName}";
    }

    public SqliteConnection Connection => new(_databaseName);

    public static DisposableDatabase Create()
    {
        string databaseName = Guid.NewGuid().ToString("n").Replace("-", ""); // Remove hyphens for cleaner filenames
        return new DisposableDatabase(databaseName);
    }

    public void Dispose()
    {
        if (File.Exists(_databaseName))
        {
            File.Delete(_databaseName);
        }
    }
}

public class EventStoreFactory : IDisposable
{
    private readonly List<DisposableDatabase> _databases = [];

    public Task<EventStore<TPayload>> GetFreshEventStore<TPayload>()
    {
        var database = DisposableDatabase.Create();
        _databases.Add(database);
        return EventStore<TPayload>.CreateAsync(database.Connection);
    }
    public void Dispose()
    {
        foreach (var database in _databases)
        {
            database.Dispose();
        }
    }
}