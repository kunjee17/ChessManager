using System;
using Avalonia.Controls;
using Avalonia.ReactiveUI;
using ChessManager.ViewModels;
using ReactiveUI;

namespace ChessManager.Views;

public partial class MainWindow : ReactiveWindow<Window>
{
    public MainWindow()
    {
        this.WhenActivated(disposables => { });
        InitializeComponent();
    }
}

public class AppViewLocator : ReactiveUI.IViewLocator
{
    public IViewFor ResolveView<T>(T viewModel, string contract = null) => viewModel switch
    {
        HomeViewModel context => new HomeView { DataContext = context },
        TournamentViewModel context => new TournamentView { DataContext = context },
        _ => throw new ArgumentOutOfRangeException(nameof(viewModel))
    };
}