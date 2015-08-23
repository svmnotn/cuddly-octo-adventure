namespace COA.Game.UI.Controls {
  using System;
  using System.Collections.Generic;
  using System.Drawing;
  using System.Windows.Forms;
  using Data;

  internal partial class QuestionSelect : UserControl {

    internal QuestionSelect(List<Topic> topics) {
      InitializeComponent();
      SetLabel();
      SetQuestions(topics);
    }

    void SetLabel() {
      var team = Program.main.archive.settings.teams[Program.main.currTeam];
      teamLabel.Font = team.font;
      teamLabel.ForeColor = team.color;
      teamLabel.Text = "Team: " + team.name + Environment.NewLine;
      teamLabel.Text += "Score: " + team.score;
    }

    void SetQuestions(List<Topic> topics) {
      questionsTable.Controls.Clear();
      questionsTable.RowStyles.Clear();
      questionsTable.ColumnStyles.Clear();
      questionsTable.ColumnCount = topics.Count;
      int columnIndex = 0;
      foreach(var t in topics) {// Add a column per topic
        int rowIndex = 0;
        questionsTable.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, (100f / topics.Count)));
        var rowPercent = 100f / (t.questions.Keys.Count + 1f);
        questionsTable.RowStyles.Add(new RowStyle(SizeType.Percent, rowPercent));
        var title = new Label();
        title.Text = t.name;
        title.TextAlign = ContentAlignment.MiddleCenter;
        title.Dock = DockStyle.Fill;
        questionsTable.Controls.Add(title, columnIndex, rowIndex);
        rowIndex += 1;
        foreach(var k in t.questions.Keys) {// Add a row per question value
          var q = t.GetQuestion(k);
          if(q != null) {
            questionsTable.RowStyles.Add(new RowStyle(SizeType.Percent, rowPercent));
            var bttn = new Button();
            bttn.Dock = DockStyle.Fill;
            bttn.BackColor = Color.LightGray;
            bttn.Text = "" + q.value;
            bttn.Click += (object sender, EventArgs e) => {
              var mode = new QuestionPanel(q);
              mode.Dock = DockStyle.Fill;
              Program.main.Current = mode;
            };
            questionsTable.Controls.Add(bttn, columnIndex, rowIndex);
            rowIndex += 1;
          }
        }
        columnIndex += 1;
      }
    }
  }
}