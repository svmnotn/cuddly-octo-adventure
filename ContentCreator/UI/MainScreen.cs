﻿namespace COA.ContentCreator.UI {
  using System;
  using System.Collections.Generic;
  using System.ComponentModel;
  using System.Windows.Forms;
  using Controls;
  using Data;

  internal partial class MainScreen : Form {
    internal Archive archive;
    internal Question currentQuestion;
    internal string CurrentTopic { get { return currentQuestion != null ? archive.GetTopicFromQuestion(currentQuestion).name : ""; } }

    internal MainScreen() : this(Archive.Default) { }

    internal MainScreen(Archive a) {
      InitializeComponent();
      archive = a;
      LoadData();
    }

    #region Menu Operations
    private void ReloadArchive(object sender, EventArgs e) {
      archive = ArchiveManager.LoadArchiveFromDir(Program.WorkingDirectory);
    }

    private void SaveArchive(object sender, EventArgs e) {
      ArchiveManager.SaveArchiveToDir(Program.WorkingDirectory, archive);
    }

    private void ImportArchive(object sender, EventArgs e) {
      var res = openArchive.ShowDialog();
      if(res == DialogResult.OK || res == DialogResult.Yes) {
        archive = ArchiveManager.LoadArchiveFromFile(openArchive.OpenFile(), Program.WorkingDirectory);
      }
    }

    private void ExportArchive(object sender, EventArgs e) {
      var res = saveArchiveDialog.ShowDialog();
      if(res == DialogResult.OK || res == DialogResult.Yes) {
        ArchiveManager.SaveArchiveToFile(saveArchiveDialog.FileName, archive, Program.WorkingDirectory);
      }
    }
    #endregion

    #region Set UI from Archive
    internal void LoadData() {
      // Load General Page
      LoadArchiveInfo();
      LoadMessageInfo();
      LoadSoundSettings();
      LoadTimeSettings();
      LoadBackgroundSettings();
      // Load Question 
      UpdateTopicTree();
      // Load Teams
      LoadTeams();
    }

    #region Set General Tab Data to Archive Info
    internal void LoadArchiveInfo() {
      archiveName.Text = archive.name ?? "";
      archiveDesc.Text = archive.description ?? "";
      version.Text = archive.version ?? "";
      author.Text = archive.author ?? "";
      license.Text = archive.license ?? "";
      updateURL.Text = archive.updateURL ?? "";
    }

    internal void LoadMessageInfo() {
      curScore.Text = archive.settings.messages.scorePresenter ?? "";
      gameOver.Text = archive.settings.messages.gameOver ?? "";
      teams.Text = archive.settings.messages.teams ?? "";
      scores.Text = archive.settings.messages.scores ?? "";
      pointsText.Text = archive.settings.messages.fakePointsName ?? "";
    }

    internal void LoadSoundSettings() {
      bkgSound.Text = archive.settings.sound.backgroundSoundLoc ?? "";
      correctSound.Text = archive.settings.sound.correctSoundLoc ?? "";
      wrongSound.Text = archive.settings.sound.wrongSoundLoc ?? "";
    }

    internal void LoadTimeSettings() {
      timerFont.Text = archive.settings.timerFont != null ? archive.settings.timerFont.ToString() : "";
      timerColor.Text = archive.settings.timerColor != null ? archive.settings.timerColor.ToString() : "";
      beforeAnswer.Value = archive.settings.time.beforeAnswer;
      afterAnswer.Value = archive.settings.time.afterAnswer;
      delayScore.Value = archive.settings.time.scoreDelay;
      delayGO.Value = archive.settings.time.gameOverDelay;
    }

    internal void LoadBackgroundSettings() {
      bkgColor.Text = archive.settings.backgroundColor.ToString();
      if(!string.IsNullOrWhiteSpace(archive.settings.backgroundLoc)) {
        bkgImage.Image = archive.settings.background ?? Extensions.LoadImage(Program.WorkingDirectory, archive.settings.backgroundLoc);
      } else {
        bkgImage.Image = null;
      }
    }
    #endregion

    internal void UpdateTopicTree() {
      topicsTree.BeginUpdate();
      topicsTree.Nodes.Clear();
      foreach(var t in archive.topics) {
        var n = topicsTree.Nodes.Add(t.name, t.name);
        foreach(var qList in t.questions.Values) {
          foreach(var q in qList) {
            var name = "[" + q.value + "] " + q.id;
            n.Nodes.Add(name, name);
          }
        }
      }
      topicsTree.EndUpdate();
      topicsTree.ExpandAll();
    }

    internal void LoadQuestion() {
      if(currentQuestion != null) {
        question.Text = currentQuestion.question ?? "";
        points.Value = currentQuestion.value;
        if(!string.IsNullOrWhiteSpace(currentQuestion.imageLoc)) {
          image.Image = currentQuestion.image ?? Extensions.LoadImage(Program.WorkingDirectory, currentQuestion.imageLoc);
        } else {
          image.Image = null;
        }
        LoadAnswers();
      }
    }

    internal void LoadAnswers() {
      if(currentQuestion != null) {
        if(currentQuestion.answers != null) {
          answersTable.Controls.Clear();
          answersTable.RowStyles.Clear();
          if(currentQuestion.answers.Count > 0) {
            foreach(var ans in currentQuestion.answers) {
              var c = new AnswerControl(ans);
              c.Dock = DockStyle.Fill;
              answersTable.Controls.Add(c);
            }
          }
        } else {
          currentQuestion.answers = new List<Answer>();
        }
      }
    }

    internal void LoadTeams() {
      teamsTable.Controls.Clear();
      teamsTable.RowStyles.Clear();
      foreach(var t in archive.settings.teams) {
        var c = new TeamControl(t);
        c.Dock = DockStyle.Fill;
        teamsTable.Controls.Add(c);
      }
    }

    #endregion

    #region Set UI to Archive

    #region Set Images
    private void SelectBkgImage(object sender, EventArgs e) {
      ShowOpenDialog(openImage, "Select the Background Image", (object send, CancelEventArgs er) => archive.settings.backgroundLoc = Program.CopyTo(((OpenFileDialog)send).FileName, Program.WorkingDirectory, "BackgroundImage"));
      if(!string.IsNullOrWhiteSpace(archive.settings.backgroundLoc)) {
        archive.settings.background = Extensions.LoadImage(Program.WorkingDirectory, archive.settings.backgroundLoc);
        var obj = (PictureBox)sender;
        obj.Image = archive.settings.background;
      }
    }

    private void SetImage(object sender, EventArgs e) {
      if(currentQuestion != null) {
        ShowOpenDialog(openImage, "Select the Question Image", (object send, CancelEventArgs er) => currentQuestion.imageLoc = ((OpenFileDialog)send).FileName);
        currentQuestion.imageLoc = Program.CopyTo(currentQuestion.imageLoc, Program.WorkingDirectory, CurrentTopic + '\\' + currentQuestion.id);
        if(!string.IsNullOrWhiteSpace(currentQuestion.imageLoc)) {
          image.Image = currentQuestion.image ?? Extensions.LoadImage(Program.WorkingDirectory, currentQuestion.imageLoc);
        }
      }
    }
    #endregion

    #region Set Text
    private void SetArchiveName(object sender, EventArgs e) {
      var obj = (TextBox)sender;
      archive.name = obj.Text;
    }

    private void SetArchiveDesc(object sender, EventArgs e) {
      var obj = (TextBox)sender;
      archive.description = obj.Text;
    }

    private void SetArchiveURL(object sender, EventArgs e) {
      var obj = (TextBox)sender;
      archive.updateURL = obj.Text;
    }

    private void SetArchiveVersion(object sender, EventArgs e) {
      var obj = (TextBox)sender;
      archive.version = obj.Text;
    }

    private void SetAuthor(object sender, EventArgs e) {
      var obj = (TextBox)sender;
      archive.author = obj.Text;
    }

    private void SetLicense(object sender, EventArgs e) {
      var obj = (TextBox)sender;
      archive.license = obj.Text;
    }

    private void SetCurrentScoreText(object sender, EventArgs e) {
      var obj = (TextBox)sender;
      archive.settings.messages.scorePresenter = obj.Text;
    }

    private void SetGameOverText(object sender, EventArgs e) {
      var obj = (TextBox)sender;
      archive.settings.messages.gameOver = obj.Text;
    }

    private void SetTeamsText(object sender, EventArgs e) {
      var obj = (TextBox)sender;
      archive.settings.messages.teams = obj.Text;
    }

    private void SetScoresText(object sender, EventArgs e) {
      var obj = (TextBox)sender;
      archive.settings.messages.scores = obj.Text;
    }

    private void SetPointsText(object sender, EventArgs e) {
      var obj = (TextBox)sender;
      archive.settings.messages.fakePointsName = obj.Text;
    }

    private void SetQuestion(object sender, EventArgs e) {
      if(currentQuestion != null) {
        currentQuestion.question = question.Text;
      }
    }

    #region Set Sounds
    private void SelectBkgSound(object sender, EventArgs e) {
      ShowOpenDialog(openSound, "Select the Background Sound", (object send, CancelEventArgs er) => archive.settings.sound.backgroundSoundLoc = Program.CopyTo(((OpenFileDialog)send).FileName, Program.WorkingDirectory, "Background"));
      var obj = (TextBox)sender;
      obj.Text = archive.settings.sound.backgroundSoundLoc;
    }

    private void SelectCorrectSound(object sender, EventArgs e) {
      ShowOpenDialog(openSound, "Select the Correct Sound", (object send, CancelEventArgs er) => archive.settings.sound.correctSoundLoc = Program.CopyTo(((OpenFileDialog)send).FileName, Program.WorkingDirectory, "Correct"));
      var obj = (TextBox)sender;
      obj.Text = archive.settings.sound.correctSoundLoc;
    }

    private void SelectWrongSound(object sender, EventArgs e) {
      ShowOpenDialog(openSound, "Select the Wrong Sound", (object send, CancelEventArgs er) => archive.settings.sound.wrongSoundLoc = Program.CopyTo(((OpenFileDialog)send).FileName, Program.WorkingDirectory, "Wrong"));
      var obj = (TextBox)sender;
      obj.Text = archive.settings.sound.wrongSoundLoc;
    }
    #endregion

    #region Set Colors
    private void SelectBkgColor(object sender, EventArgs e) {
      colorDialog.ShowDialog();
      var obj = (TextBox)sender;
      archive.settings.backgroundColor = colorDialog.Color;
      obj.Text = colorDialog.Color.ToString();
    }

    private void SelectTimerColor(object sender, EventArgs e) {
      colorDialog.ShowDialog();
      var obj = (TextBox)sender;
      archive.settings.timerColor = colorDialog.Color;
      obj.Text = colorDialog.Color.ToString();
    }
    #endregion

    #region Set Fonts
    private void SelectTimerFont(object sender, EventArgs e) {
      fontDialog.ShowDialog();
      archive.settings.timerFont = fontDialog.Font;
      var obj = (TextBox)sender;
      obj.Text = archive.settings.timerFont.ToString();
    }
    #endregion

    #endregion

    #region Set Numbers
    private void SetBeforeQuestion(object sender, EventArgs e) {
      var obj = (NumericUpDown)sender;
      archive.settings.time.beforeAnswer = obj.Value;
    }

    private void SetAfterQuestion(object sender, EventArgs e) {
      var obj = (NumericUpDown)sender;
      archive.settings.time.afterAnswer = obj.Value;
    }

    private void SetScoreDelay(object sender, EventArgs e) {
      var obj = (NumericUpDown)sender;
      archive.settings.time.scoreDelay = obj.Value;
    }

    private void SetGODelay(object sender, EventArgs e) {
      var obj = (NumericUpDown)sender;
      archive.settings.time.gameOverDelay = obj.Value;
    }

    private void SetPoints(object sender, EventArgs e) {
      if(currentQuestion != null) {
        var t = archive.GetTopicFromQuestion(currentQuestion);
        t.ChangeValue(currentQuestion, (int)points.Value);
        UpdateTopicTree();
      }
    }
    #endregion

    #region Handle Button Input
    private void AddTopic(object sender, EventArgs e) {
      using(var dig = new InputDialog("Topic", "Please type the Topic's Title")) {
        if(dig.ShowDialog(this) == DialogResult.OK) {
          var topic = new Topic();
          topic.name = dig.userInput.Text;
          topic.questions = new Dictionary<int, List<Question>>();
          archive.topics.Add(topic);
          UpdateTopicTree();
        }
      }
    }

    private void RemoveTopic(object sender, EventArgs e) {
      if(topicsTree.SelectedNode != null) {
        if(topicsTree.SelectedNode.Parent == null) {
          archive.topics.Remove(archive.GetTopicFromName(topicsTree.SelectedNode.Name));
          UpdateTopicTree();
        }
      }
    }

    private void AddQuestion(object sender, EventArgs e) {
      using(var dig = new InputDialog("Question", "Please type the Question's ID (This will only be shown in the Content Creator)")) {
        if(dig.ShowDialog(this) == DialogResult.OK) {
          if(topicsTree.SelectedNode != null) {
            var q = new Question();
            q.answers = new List<Answer>();
            q.id = dig.userInput.Text;
            if(topicsTree.SelectedNode.Parent == null) {
              var t = archive.GetTopicFromName(topicsTree.SelectedNode.Name);
              t.AddQuestion(q);
            } else {
              var t = archive.GetTopicFromName(topicsTree.SelectedNode.Parent.Name);
              t.AddQuestion(q);
            }
            UpdateTopicTree();
          }
        }
      }
    }

    private void RemoveQuestion(object sender, EventArgs e) {
      if(topicsTree.SelectedNode != null) {
        if(topicsTree.SelectedNode.Parent != null) {
          archive.GetTopicFromName(topicsTree.SelectedNode.Parent.Name).RemoveQuestionByID(RemoveValue(topicsTree.SelectedNode));
          UpdateTopicTree();
        }
      }
    }

    private void AddAnswer(object sender, EventArgs e) {
      if(currentQuestion != null) {
        if(currentQuestion.answers != null) {
          currentQuestion.answers.Add(new Answer());
          LoadAnswers();
        }
      }
    }

    private void AddTeam(object sender, EventArgs e) {
      archive.settings.teams.Add(new Team());
      LoadTeams();
    }
    #endregion

    #endregion

    #region Handle Tree View
    private void BeforeNodeNameChange(object sender, NodeLabelEditEventArgs e) {
      if(IsQuestion(e.Node)) {
        e.Node.Text = RemoveValue(e.Node);
      }
    }

    private void AfterNodeNameChange(object sender, NodeLabelEditEventArgs e) {
      if(IsQuestion(e.Node)) {
        var txt = AddValue(e.Node, e.Label);
        e.Node.Name = txt;
        e.Node.Text = txt;
      }
    }

    private void NodeClicked(object sender, TreeNodeMouseClickEventArgs e) {
      if(IsQuestion(e.Node)) {
        currentQuestion = archive.GetTopicFromName(e.Node.Parent.Name).GetQuestionFromID(RemoveValue(e.Node));
        LoadQuestion();
      } else {
        currentQuestion = null;
      }
    }
    #endregion

    #region Utility Methods
    void ShowOpenDialog(FileDialog dialog, string title, CancelEventHandler eventH) {
      dialog.Title = title;
      dialog.FileOk += eventH;
      dialog.ShowDialog();
      dialog.Title = "";
      dialog.FileOk -= eventH;
    }

    string RemoveValue(TreeNode n) {
      var txt = n.Text.Remove(n.Text.IndexOf('['), n.Text.IndexOf(']') + 1);
      return txt.TrimStart();
    }

    string AddValue(TreeNode n, string extra) {
      return n.Name.Remove(n.Name.IndexOf(']') + 1) + " " + extra;
    }

    bool IsQuestion(TreeNode n) {
      return n.Name.StartsWith("[");
    }

    private void OnClose(object sender, FormClosingEventArgs e) {
      ArchiveManager.SaveArchiveToDir(Program.WorkingDirectory, archive);
    }
    #endregion
  }
}