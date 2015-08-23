namespace COA.Game.UI.Controls {
  using System;
  using System.Drawing;
  using System.Windows.Forms;
  using Data;

  internal partial class QuestionPanel : UserControl {
    Question question;
    // timer variables
    decimal max;
    Action next;

    internal QuestionPanel(Question q) {
      InitializeComponent();
      question = q;
      SetTeamLabel();
      SetTimerLabel();
      SetQuestion();
      SetTimer();
    }

    void SetTeamLabel() {
      var team = Program.main.archive.settings.teams[Program.main.currTeam];
      teamLabel.Font = team.font;
      teamLabel.ForeColor = team.color;
      teamLabel.Text = "Team: " + team.name + Environment.NewLine;
      teamLabel.Text += "Score: " + team.score;
    }

    void SetTimerLabel() {
      timerLabel.Font = Program.main.archive.settings.timerFont;
      timerLabel.ForeColor = Program.main.archive.settings.timerColor;
      timerLabel.Text = "Time Left: " + Program.main.archive.settings.time.beforeAnswer;
    }

    void SetQuestion() {
      if(!string.IsNullOrWhiteSpace(question.imageLoc)) {
        image.Image = question.image ?? Extensions.LoadImage(Program.ArchivePath(Program.main.archive), question.imageLoc);
      }
      text.Text = question.question;
      SetAnswers();
    }

    void SetAnswers() {
      answersTable.Controls.Clear();
      answersTable.RowStyles.Clear();
      question.answers.Shuffle();
      foreach(var a in question.answers) {
        answersTable.RowStyles.Add(new RowStyle(SizeType.Percent, (100f / question.answers.Count)));
        var bttn = new AnswerButton(a.correct);
        bttn.Name = "" + a.correct;
        bttn.Dock = DockStyle.Fill;
        bttn.BackColor = Color.LightGray;
        bttn.Text = a.answer;
        if(!string.IsNullOrWhiteSpace(a.imageLoc)) {
          bttn.Image = a.image ?? Extensions.LoadImage(Program.ArchivePath(Program.main.archive), question.imageLoc);
        }
        bttn.Click += (object sender, EventArgs e) => {
          if(a.correct) {
            // Add Points
            Program.main.archive.settings.teams[Program.main.currTeam].score += question.value;
            // Play CorrectSound
            if(Program.main.yes != null) {
              Program.main.yes.Play();
            }
          } else {
            // Play Wrong Sound
            if(Program.main.no != null) {
              Program.main.no.Play();
            }
          }
          SetRightTimer();
        };
        answersTable.Controls.Add(bttn);
      }
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
      Program.main.timer.Interval = 1000;
      max = Program.main.archive.settings.time.afterAnswer;
      next = GoResult;
      Program.main.timerFire += UpdateLabel;
      Program.main.timer.Enabled = true;
      Program.main.timer.Start();
    }

    void GoResult() {
      // Cancel the Timer
      Program.main.timer.Stop();
      Program.main.timerFire -= UpdateLabel;
      Program.main.timer.Enabled = false;
      // Take question out of the poool and Shuffle the pool
      Program.main.archive.RemoveQuestion(question);
      Program.main.archive.ShuffleTopics();
      // Go to result
      var mode = new Result();
      mode.Dock = DockStyle.Fill;
      Program.main.Current = mode;
    }

    void SetTimer() {
      Program.main.timer.Interval = 1000;
      max = Program.main.archive.settings.time.beforeAnswer;
      next = SetRightTimer;
      Program.main.timerFire += UpdateLabel;
      Program.main.timer.Enabled = true;
      Program.main.timer.Start();
    }

    void UpdateLabel() {
      Program.main.timerCount += 1;
      var left = max - Program.main.timerCount;
      timerLabel.Text = "Time Left: " + left;
      if(left <= 0) {
        Program.main.timerCount = 0;
        next();
      }
    }
  }
}