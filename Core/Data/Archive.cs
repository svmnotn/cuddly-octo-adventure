namespace COA.Data {
  using System;
  using System.Collections.Generic;

  public class Archive {
    public string name;
    public string description;
    public string version;
    public string author;
    public string license;
    public string updateURL;
    public string teamWinningMsg;
    public string tyingMsg;
    public string teamLosingMsg;
    public string teamWonMsg;
    public string teamLostMsg;
    public string tiedMsg;
    [NonSerialized]
    public List<Topic> topics;
    [NonSerialized]
    public Settings settings;

    public Topic GetTopicFromName(string name) {
      foreach(var t in topics) {
        if(t.name.Equals(name)) {
          return t;
        }
      }
      return null;
    }

    public Topic GetTopicFromQuestion(Question q) {
      foreach(var t in topics) {
        if(t.ContainsQuestion(q)) {
          return t;
        }
      }
      return null;
    }

    public void RemoveQuestion(Question q) {
      foreach(var t in topics) {
        if(t.RemoveQuestion(q)) {
          return;
        }
      }
    }

    public void ShuffleTopics() {
      foreach(var t in topics) {
        t.ShuffleAll();
      }
    }

    public bool HasQuestions() {
      int count = 0;
      foreach(var t in topics) {
        if(t.HasQuestions()) {
          count += 1;
        }
      }
      return count > 0;
    }

    public static Archive Default {
      get {
        var tmp = new Archive();
        #region Archive Main Data
        tmp.name = "Default Archive";
        tmp.description = "This is the Default that is automatically created when the Cuddly Octo Adventure Content Creator is first run.";
        tmp.version = "1.0.0";
        tmp.author = "someone";
        tmp.license = "Public Domain";
        tmp.teamWinningMsg = "{team} is winning.";
        tmp.teamLosingMsg = "{team} is losing.";
        tmp.tyingMsg = "{teams} are tied!";
        tmp.teamWonMsg = "{team} Won!";
        tmp.teamLostMsg = "{team} Lost.";
        tmp.tiedMsg = "{teams} tied";
        tmp.topics = new List<Topic>();
        #endregion
        var t = new Topic();
        #region Topic 1
        t.name = "Test Topic";
        t.questions = new Dictionary<int, List<Question>>();
        var q = new Question();
        #region question 1
        q.id = "test1";
        q.value = 100;
        q.question = "Is this a test?";
        q.answers = new List<Answer>();
        var a = new Answer();
        a.answer = "Yes!";
        a.correct = true;
        q.answers.Add(a);
        a = new Answer();
        a.answer = "No!";
        q.answers.Add(a);
        #endregion
        t.AddQuestion(q);
        q = new Question();
        #region question 2
        q.id = "test2";
        q.value = 100;
        q.question = "Is this not a test?";
        q.answers = new List<Answer>();
        a = new Answer();
        a.answer = "No!";
        a.correct = true;
        q.answers.Add(a);
        a = new Answer();
        a.answer = "Yes!";
        q.answers.Add(a);
        #endregion
        t.AddQuestion(q);
        q = new Question();
        #region question 3
        q.id = "test3";
        q.value = 1000;
        q.question = "Is this not not a test?";
        q.answers = new List<Answer>();
        a = new Answer();
        a.answer = "Yes!";
        a.correct = true;
        q.answers.Add(a);
        a = new Answer();
        a.answer = "No!";
        q.answers.Add(a);
        #endregion
        t.AddQuestion(q);
        q = new Question();
        #region question 4
        q.id = "test4";
        q.value = 1000;
        q.question = "Is a game?";
        q.answers = new List<Answer>();
        a = new Answer();
        a.answer = "Yes!";
        a.correct = true;
        q.answers.Add(a);
        a = new Answer();
        a.answer = "No!";
        q.answers.Add(a);
        #endregion
        t.AddQuestion(q);
        q = new Question();
        #region question 5
        q.id = "test5";
        q.value = 100;
        q.question = "Am I awesome?";
        q.answers = new List<Answer>();
        a = new Answer();
        a.answer = "Yes!";
        a.correct = true;
        q.answers.Add(a);
        a = new Answer();
        a.answer = "No!";
        q.answers.Add(a);
        #endregion
        t.AddQuestion(q);
        q = new Question();
        #region question 6
        q.id = "test6";
        q.value = 1000;
        q.question = "Am I da Bomb?";
        q.answers = new List<Answer>();
        a = new Answer();
        a.answer = "Yes!";
        a.correct = true;
        q.answers.Add(a);
        a = new Answer();
        a.answer = "No!";
        q.answers.Add(a);
        #endregion
        t.AddQuestion(q);
        #endregion
        tmp.topics.Add(t);
        t = new Topic();
        #region Topic 2
        t.name = "Trial Topic";
        t.questions = new Dictionary<int, List<Question>>();
        q = new Question();
        #region question 1
        q.id = "trial1";
        q.value = 100;
        q.question = "42?";
        q.answers = new List<Answer>();
        a = new Answer();
        a.answer = "Yes!";
        a.correct = true;
        q.answers.Add(a);
        a = new Answer();
        a.answer = "No!";
        q.answers.Add(a);
        #endregion
        t.AddQuestion(q);
        q = new Question();
        #region question 2
        q.id = "trial2";
        q.value = 100;
        q.question = "life?";
        q.answers = new List<Answer>();
        a = new Answer();
        a.answer = "Yes!";
        a.correct = true;
        q.answers.Add(a);
        a = new Answer();
        a.answer = "yes";
        q.answers.Add(a);
        #endregion
        t.AddQuestion(q);
        q = new Question();
        #region question 3
        q.id = "trial3";
        q.value = 1000;
        q.question = "food?";
        q.answers = new List<Answer>();
        a = new Answer();
        a.answer = "Yes!";
        a.correct = true;
        q.answers.Add(a);
        a = new Answer();
        a.answer = "No!";
        q.answers.Add(a);
        #endregion
        t.AddQuestion(q);
        q = new Question();
        #region question 4
        q.id = "trial4";
        q.value = 1000;
        q.question = "Grand Strategies?";
        q.answers = new List<Answer>();
        a = new Answer();
        a.answer = "Yes!";
        a.correct = true;
        q.answers.Add(a);
        a = new Answer();
        a.answer = "No!";
        q.answers.Add(a);
        #endregion
        t.AddQuestion(q);
        q = new Question();
        #region question 5
        q.id = "trial5";
        q.value = 100;
        q.question = "Fun times?";
        q.answers = new List<Answer>();
        a = new Answer();
        a.answer = "Yes!";
        a.correct = true;
        q.answers.Add(a);
        a = new Answer();
        a.answer = "No!";
        q.answers.Add(a);
        #endregion
        t.AddQuestion(q);
        q = new Question();
        #region question 6
        q.id = "trial6";
        q.value = 1000;
        q.question = "Books?";
        q.answers = new List<Answer>();
        a = new Answer();
        a.answer = "Yes!";
        a.correct = true;
        q.answers.Add(a);
        a = new Answer();
        a.answer = "No!";
        q.answers.Add(a);
        #endregion
        t.AddQuestion(q);
        #endregion
        tmp.topics.Add(t);
        tmp.settings = Settings.Default;
        return tmp;
      }
    }
  }
}