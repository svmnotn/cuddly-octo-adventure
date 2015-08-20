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
        if(!Controls.Contains(value)) {
          Controls.Add(value);
        }
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
    internal readonly Archive archive;
    internal GameMode gm;
    internal readonly Team[] teams;
    internal int currTeam;
    internal Action timerFire;
    internal int timerCount;


    internal MainWindow() : this(Archive.Default) { }

    internal MainWindow(Archive a) {
      InitializeComponent();
      archive = a;
      teams = archive.settings.teams.ToArray();
      Text = a.name;
      var mode = new ModeSelect();
      mode.Dock = DockStyle.Fill;
      Current = mode;
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