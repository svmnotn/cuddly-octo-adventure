namespace COA.Data {
  using System;
  using System.Drawing;

  public class Team : IComparable<Team> {
    public string name;
    public Color color;
    public Font font;
    [NonSerialized]
    public int score;

    public int CompareTo(Team other) {
      if(score > other.score) {
        return -1;
      } else if(score < other.score) {
        return 1;
      }
      return 0;
    }
  }
}