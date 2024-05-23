using System.Reactive;
using NanoidDotNet;
using ReactiveUI;

namespace ChessManager.ViewModels;

public class TournamentViewModel(IScreen screen) : ViewModelBase, IRoutableViewModel
{
    public string? UrlPathSegment { get; } = Nanoid.Generate(size: 5);
    public IScreen HostScreen { get; } = screen;

    public ReactiveCommand<Unit, IRoutableViewModel> GoHome { get; } =
        ReactiveCommand.CreateFromObservable(() => screen.Router.Navigate.Execute(new HomeViewModel(screen)));
}