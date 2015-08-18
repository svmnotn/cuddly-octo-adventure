namespace COA.Game {
  using System;
  using System.Collections.Generic;

  internal class GameMode {
    internal static readonly GameMode Classical = new GameMode("Classical", "Lets you select a topic and question", delegate(object sender, EventArgs e) {
      // Classical Start Logic



    });
    internal static readonly GameMode RandomTopic = new GameMode("Random Topics", "Selects a randomized topic, and lets you select the question", delegate(object sender, EventArgs e) {
      // RandomTopic Start Logic



    });
    internal static readonly GameMode RandomQuestion = new GameMode("Random Questions", "Selects a randomized question from any topic", delegate(object sender, EventArgs e) {
      // RandomQuestion Start Logic



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
