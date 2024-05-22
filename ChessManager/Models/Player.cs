using System;

namespace ChessManager.Models;

public record Player(Guid Id, string Name, int Rating, string Country);