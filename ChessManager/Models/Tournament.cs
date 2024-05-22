using System;
using System.Collections.Generic;

namespace ChessManager.Models;

public record Tournament(Guid Id, string Name, string Location, DateTime StartDate, DateTime EndDate);