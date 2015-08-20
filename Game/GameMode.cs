namespace COA.Game {
  using System;
  using System.Collections.Generic;
  using System.Windows.Forms;
  using UI.Controls;

  internal class GameMode {
    internal static readonly GameMode Classical = new GameMode("Classical", "Lets you select a topic and question", (object sender, EventArgs e) => {
      // Classical Start Logic
      Program.main.gm = Classical;
      Console.WriteLine("Classic");
      var mode = new QuestionSelect(Program.main.archive.topics);
      mode.Dock = DockStyle.Fill;
      Program.main.Current = mode;
    });
    internal static readonly GameMode RandomTopic = new GameMode("Random Topics", "Selects a randomized topic, and lets you select the question", (object sender, EventArgs e) => {
      // Random Topic Start Logic
      Program.main.gm = RandomTopic;
      Console.WriteLine("RNG Topic");
      Program.main.archive.topics.Shuffle();
      var tmp = new List<Data.Topic>();
      tmp.Add(Program.main.archive.topics[0]);
      var mode = new QuestionSelect(tmp);
      mode.Dock = DockStyle.Fill;
      Program.main.Current = mode;
    });
    internal static readonly GameMode RandomQuestion = new GameMode("Random Questions", "Selects a randomized question from any topic", (object sender, EventArgs e) => {
      // Random Question Start Logic
      Program.main.gm = RandomQuestion;
      Console.WriteLine("RNG Question");
      Program.main.archive.topics.Shuffle();
      var mode = new Question(Program.main.archive.topics[0].GetQuestion());
      mode.Dock = DockStyle.Fill;
      Program.main.Current = mode;
    });

    internal readonly string Name;
    internal readonly string Description;
    internal readonly EventHandler Start;

    private GameMode(string name, string desc, EventHandler start) {
      Name = name;
      Description = desc;
      Start = start;
    }

    public static IEnumerable<GameMode> Values {
      get {
        yield return Classical;
        yield return RandomTopic;
        yield return RandomQuestion;
      }
    }
  }
}
