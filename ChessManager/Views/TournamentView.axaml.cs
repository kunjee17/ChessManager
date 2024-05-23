using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using Avalonia.ReactiveUI;
using ChessManager.ViewModels;

namespace ChessManager.Views;

public partial class TournamentView : ReactiveUserControl<TournamentViewModel>
{
    public TournamentView()
    {
        InitializeComponent();
    }
}