using NanoidDotNet;
using ReactiveUI;

namespace ChessManager.ViewModels;

public class HomeViewModel(IScreen screen) : ViewModelBase, IRoutableViewModel
{
    public string? UrlPathSegment { get; } = Nanoid.Generate(size: 5);
    public IScreen HostScreen { get; } = screen;
}