using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using Avalonia.ReactiveUI;
using ChessManager.ViewModels;
using ReactiveUI;

namespace ChessManager.Views;

public partial class HomeView : ReactiveUserControl<HomeViewModel>
{
    public HomeView()
    {
        this.WhenActivated(disposables => { });
        InitializeComponent();
    }
}