using System;

namespace ChessManager.Models;

public enum ChessColor
{
    White,
    Black
}

public record MatchPlayer(Guid PlayerId, ChessColor SelectedColor);