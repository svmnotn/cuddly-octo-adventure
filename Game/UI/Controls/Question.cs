namespace COA.Game.UI.Controls {
  using System;
  using System.Windows.Forms;

  internal partial class Question : UserControl {
    Data.Question question;
    internal Question(Data.Question q) {
      InitializeComponent();
      question = q;
      if(!string.IsNullOrWhiteSpace(question.imageLoc)) {
        image.Image = question.image ?? Extensions.LoadImage(Program.ArchivePath(Program.main.archive), question.imageLoc);
      }
      text.Text = question.question;
      answersTable.Controls.Clear();
      answersTable.RowStyles.Clear();
      foreach(var a in question.answers) {
        answersTable.RowStyles.Add(new RowStyle(SizeType.Percent, (100f / question.answers.Count)));
        var bttn = new AnswerButton(a.correct);
        bttn.Name = "" + a.correct;
        bttn.Dock = DockStyle.Fill;
        bttn.Text = a.answer;
        if(!string.IsNullOrWhiteSpace(a.imageLoc)) {
          bttn.Image = a.image ?? Extensions.LoadImage(Program.ArchivePath(Program.main.archive), question.imageLoc);
        }
        bttn.Click += (object sender, EventArgs e) => {
          Console.WriteLine("ButtonClick!");
          bttn.Reveal();
          if(a.correct) {
            Program.main.teams[Program.main.currTeam++].score += question.value;
          } else {
            Console.WriteLine("wrong!");
            foreach(var c in answersTable.Controls.Find("true", true)) {
              var b = (AnswerButton)c;
              b.Reveal();
            }
          }
          // Move to Result
        };
        answersTable.Controls.Add(bttn);
      }
    }
  }
}