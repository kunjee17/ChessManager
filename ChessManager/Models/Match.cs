using System;

namespace ChessManager.Models;

public record Match(Guid Id, Guid MatchPlayer1Id, Guid MatchPlayer2Id, Guid RoundId, DateTime Date, Result Result);