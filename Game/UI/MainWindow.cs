namespace COA.Game.UI {
  using System;
  using System.Windows.Forms;
  using Controls;
  using Data;

  internal partial class MainWindow : Form {
    UserControl current;
    internal UserControl Current {
      get { return current; }
      set {
        if(current != null) {
          current.Visible = false;
        }
        if(!Controls.Contains(value)) {
          Controls.Add(value);
        }
        current = value;
        current.Dock = DockStyle.Fill;
        current.Visible = true;
      }
    }
    internal readonly Archive archive;
    internal GameMode gm;
    internal int currTeam;
    internal Action timerFire;
    internal int timerCount;


    internal MainWindow() : this(Archive.Default) { }

    internal MainWindow(Archive a) {
      InitializeComponent();
      archive = a;
      Text = a.name;
      BackColor = archive.settings.backgroundColor;
      if(!string.IsNullOrWhiteSpace(archive.settings.backgroundLoc)) {
        BackgroundImage = archive.settings.background ?? Extensions.LoadImage(Program.ArchivePath(archive), archive.settings.backgroundLoc);
      }
      Current = new ModeSelect();
    }

    private void OnTick(object sender, EventArgs e) {
      if(timerFire != null) {
        timerFire();
      }
    }

    private void OnClose(object sender, FormClosedEventArgs e) {
      Application.Exit();
    }
  }
}