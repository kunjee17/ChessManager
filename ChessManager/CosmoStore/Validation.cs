using System;

namespace ChessManager.CosmoStore;

public static class VersionValidator
{
    public static void ValidateVersion(Guid streamId, long nextVersion, ExpectedVersion<long> expectedVersion)
    {
        switch (expectedVersion)
        {
            case Any<long>:
                break;
            case NoStream<long>:
                if (nextVersion > 1L)
                {
                    var nvText = $"[nextVersion={nextVersion}]";
                    throw new InvalidOperationException($"ESERROR_VERSION_STREAMEXISTS: {nvText} Stream '{streamId}' was expected to be empty, but contains {nextVersion - 1L} events");
                }
                break;
            case Exact<long> exactVersion:
                if (nextVersion != exactVersion.Version)
                {
                    var nvText = $"[nextVersion={nextVersion}]";
                    throw new InvalidOperationException($"ESERROR_VERSION_VERSIONNOTMATCH: {nvText} Stream '{streamId}' was expected to have next version {exactVersion.Version}, but has {nextVersion}");
                }
                break;
            default:
                throw new ArgumentException("Invalid expected version type");
        }
    }

    public static void ValidateVersionU(string streamId, ulong nextVersion, ExpectedVersion<ulong> expectedVersion)
    {
        switch (expectedVersion)
        {
            case Any<ulong>:
                break;
            case NoStream<ulong>:
                if (nextVersion > 1UL)
                {
                    var nvText = $"[nextVersion={nextVersion}]";
                    throw new InvalidOperationException($"ESERROR_VERSION_STREAMEXISTS: {nvText} Stream '{streamId}' was expected to be empty, but contains {nextVersion - 1UL} events");
                }
                break;
            case Exact<ulong> exactVersion:
                if (nextVersion != exactVersion.Version)
                {
                    var nvText = $"[nextVersion={nextVersion}]";
                    throw new InvalidOperationException($"ESERROR_VERSION_VERSIONNOTMATCH: {nvText} Stream '{streamId}' was expected to have next version {exactVersion.Version}, but has {nextVersion}");
                }
                break;
            default:
                throw new ArgumentException("Invalid expected version type");
        }
    }
}

