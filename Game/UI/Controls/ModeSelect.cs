namespace COA.Game.UI.Controls {
  using System.Windows.Forms;

  internal partial class ModeSelect : UserControl {

    internal ModeSelect() {
      InitializeComponent();
      modeTable.Controls.Clear();
      modeTable.RowStyles.Clear();
      modeTable.ColumnStyles.Clear();
      foreach(var mode in GameMode.Values) {
        // Dirty hack that needs to be updated if there ever is another game mode => (100/gamemodeCount)
        modeTable.RowStyles.Add(new RowStyle(SizeType.Percent, (100f/3f)));
        var bttn = new Button();
        bttn.Dock = DockStyle.Fill;
        bttn.Text = mode.Name;
        bttn.Click += mode.Start;
        modeTable.Controls.Add(bttn);
        toolTip.SetToolTip(bttn, mode.Description);
      }
    }
  }
}
