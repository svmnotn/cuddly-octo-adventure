namespace COA.Game.UI {
  using System;
  using System.Windows.Forms;
  using Data;

  internal partial class MainWindow : Form {
    Archive archive;
    UserControl current;
    internal UserControl Current {
      get { return current; }
      set {
        if(current != null) {
          current.Visible = false;
        }
        current = value;
        if(!string.IsNullOrWhiteSpace(archive.settings.backgroundLoc)) {
          current.BackgroundImage = archive.settings.background ?? Extensions.LoadImage(Program.ArchivePath(archive), archive.settings.backgroundLoc);
        }
        current.BackColor = archive.settings.backgroundColor;
        current.Visible = true;
      }
    }

    internal MainWindow() : this(Archive.Default) { }

    internal MainWindow(Archive a) {
      InitializeComponent();
      archive = a;
      Text = a.name;
    }

    private void OnTick(object sender, EventArgs e) {
      ((Timer)sender).Enabled = false;
    }

    private void OnClose(object sender, FormClosedEventArgs e) {
      Application.Exit();
    }
  }
}