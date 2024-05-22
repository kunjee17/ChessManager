using System;
using System.Collections.Generic;

namespace ChessManager.Models;

public record Round(Guid Id, int RoundNumber, Guid TournamentId);