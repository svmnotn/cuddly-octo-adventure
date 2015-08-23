namespace COA.Game.UI {
  using System;
  using System.Media;
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
    internal SoundPlayer main;
    internal SoundPlayer yes;
    internal SoundPlayer no;

    internal MainWindow() : this(Archive.Default) { }

    internal MainWindow(Archive a) {
      InitializeComponent();
      archive = a;
      if(!string.IsNullOrWhiteSpace(archive.settings.sound.backgroundSoundLoc)) {
        main = new SoundPlayer();
        main.SoundLocation = archive.settings.sound.backgroundSoundLoc.RelativeTo(Program.ArchivePath(archive));
        main.PlayLooping();
      }
      if(!string.IsNullOrWhiteSpace(archive.settings.sound.correctSoundLoc)) {
        yes = new SoundPlayer();
        yes.SoundLocation = archive.settings.sound.correctSoundLoc.RelativeTo(Program.ArchivePath(archive));
        yes.LoadAsync();
      }
      if(!string.IsNullOrWhiteSpace(archive.settings.sound.wrongSoundLoc)) {
        no = new SoundPlayer();
        no.SoundLocation = archive.settings.sound.wrongSoundLoc.RelativeTo(Program.ArchivePath(archive));
        no.LoadAsync();
      }
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