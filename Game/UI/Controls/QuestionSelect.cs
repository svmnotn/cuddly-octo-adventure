namespace COA.Game.UI.Controls {
  using System;
  using System.Collections.Generic;
  using System.Drawing;
  using System.Windows.Forms;
  using Data;

  internal partial class QuestionSelect : UserControl {

    internal QuestionSelect(List<Topic> topics) {
      Console.WriteLine(topics);
      InitializeComponent();
      questionsTable.Controls.Clear();
      questionsTable.RowStyles.Clear();
      questionsTable.ColumnStyles.Clear();
      foreach(var t in topics) {
        questionsTable.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, (100f / topics.Count)));
        var rowPercent = 100f / (t.questions.Keys.Count + 1f);
        questionsTable.RowStyles.Add(new RowStyle(SizeType.Percent, rowPercent));
        var title = new Label();
        title.Text = t.name;
        title.TextAlign = ContentAlignment.MiddleCenter;
        title.Dock = DockStyle.Fill;
        questionsTable.Controls.Add(title);
        foreach(var k in t.questions.Keys) {
          var q = t.GetQuestion(k);
          questionsTable.RowStyles.Add(new RowStyle(SizeType.Percent, rowPercent));
          var bttn = new Button();
          bttn.Dock = DockStyle.Fill;
          bttn.Text = "" + q.value;
          bttn.Click += (object sender, EventArgs e) => {
            // Go to question
          };
          questionsTable.Controls.Add(bttn);
        }
      }
    }
  }
}