namespace COA.Game.UI.Controls {
  using System;
  using System.Windows.Forms;

  internal partial class Question : UserControl {
    Data.Question question;
    internal Question(Data.Question q) {
      InitializeComponent();
      question = q;
      timerLabel.Font = Program.main.archive.settings.timerFont;
      timerLabel.ForeColor = Program.main.archive.settings.timerColor;
      timerLabel.Text = "Time Left: " + Program.main.archive.settings.time.beforeAnswer;
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
          if(a.correct) {
            Program.main.teams[Program.main.currTeam++].score += question.value;
          }
          SetRightTimer();
        };
        answersTable.Controls.Add(bttn);
      }
      SetTimer();
    }

    void RevealRight() {
      foreach(var c in answersTable.Controls.Find("true", true)) {
        var b = (AnswerButton)c;
        b.Reveal();
      }
    }

    void SetRightTimer() {
      // Cancel the Timer
      Program.main.timer.Stop();
      Program.main.timerFire -= UpdateLabel;
      Program.main.timer.Enabled = false;
      // Reveal right and wait
      RevealRight();
      Program.main.timer.Interval = (int)(Program.main.archive.settings.time.afterAnswer * 1000);
      Program.main.timerFire += GoResult;
      Program.main.timer.Enabled = true;
      Program.main.timer.Start();
    }

    void GoResult() {
      // Cancel the Timer
      Program.main.timer.Stop();
      Program.main.timerFire -= GoResult;
      Program.main.timer.Enabled = false;
      // Go to result
      var mode = new Result();
      mode.Dock = DockStyle.Fill;
      Program.main.Current = mode;
    }

    void SetTimer() {
      Program.main.timer.Interval = 1000;
      Program.main.timerFire += UpdateLabel;
      Program.main.timer.Enabled = true;
      Program.main.timer.Start();
    }

    void UpdateLabel() {
      Program.main.timerCount += 1;
      var left = Program.main.archive.settings.time.beforeAnswer - Program.main.timerCount;
      timerLabel.Text = "Time Left: " + left;
      if(left <= 0) {
        Program.main.timerCount = 0;
        SetRightTimer();
      }
    }
  }
}