using System.Reactive;
using ReactiveUI;

namespace ChessManager.ViewModels;

public class MainWindowViewModel : ViewModelBase, IScreen
{

    public RoutingState Router { get; } = new();
    public ReactiveCommand<Unit, IRoutableViewModel> GoHome { get; }
    public ReactiveCommand<Unit, IRoutableViewModel> GoTournament { get; }

    public MainWindowViewModel()
    {
        Router.Navigate.Execute(new HomeViewModel(this));
        GoHome = ReactiveCommand.CreateFromObservable(() => Router.Navigate.Execute(new HomeViewModel(this)));
        GoTournament = ReactiveCommand.CreateFromObservable(() => Router.Navigate.Execute(new TournamentViewModel(this)));
    }
}