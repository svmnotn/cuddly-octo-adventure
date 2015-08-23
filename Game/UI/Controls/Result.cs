﻿namespace COA.Game.UI.Controls {
  using System;
  using System.Collections.Generic;
  using System.Drawing;
  using System.Windows.Forms;
  using Data;

  internal partial class Result : UserControl {

    internal Result() {
      InitializeComponent();
      var info = "";
      // Check if game over
      Console.WriteLine();
      if(Program.main.archive.HasQuestions()) {// Game not Over
        // Set the Text
        label.Text = "Current scores are:";
        WriteTable();
        // Set timer
        Program.main.timer.Interval = (int)(Program.main.archive.settings.time.scoreDelay * 1000);
        Program.main.timerFire += GoNext;
        Program.main.timer.Enabled = true;
        Program.main.timer.Start();
      } else {// No more questions == Game over
        // Set the Text
        label.Text = "Tha tha tha That's All Folks!";
        WriteTable();
        // Set timer
        Program.main.timer.Interval = (int)(Program.main.archive.settings.time.gameOverDelay * 1000);
        Program.main.timerFire += End;
        Program.main.timer.Enabled = true;
        Program.main.timer.Start();
      }
      label.Text = info;
    }

    void WriteTable() {
      int teamIndex = 0;
      int scoreIndex = 1;
      int currentRow = 1;
      List<Team> teams = new List<Team>(Program.main.archive.settings.teams);
      teams.Sort();
      foreach(var t in teams) {
        // Figure out relation and write text
        var title = new Label();
        title.Text = t.name;
        title.TextAlign = ContentAlignment.MiddleCenter;
        title.Dock = DockStyle.Fill;
        table.Controls.Add(title, teamIndex, currentRow);
        var score = new Label();
        score.Text = t.score + " FIPs";
        score.TextAlign = ContentAlignment.MiddleCenter;
        score.Dock = DockStyle.Fill;
        table.Controls.Add(score, scoreIndex, currentRow);
        currentRow += 1;
      }
    }

    void GoNext() {
      // Stop timer
      Program.main.timer.Stop();
      Program.main.timerFire -= GoNext;
      Program.main.timer.Enabled = false;
      // Go to the next step
      Program.main.currTeam += 1;
      if(Program.main.currTeam >= Program.main.archive.settings.teams.Count) {
        Program.main.currTeam = 0;
      }
      Program.main.gm.Start(null, null);
    }

    void End() {
      // Stop timer
      Program.main.timer.Stop();
      Program.main.timerFire -= End;
      Program.main.timer.Enabled = false;
      // End the Game
      Application.Exit();
    }
  }
}