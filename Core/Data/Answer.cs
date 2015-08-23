namespace COA.Data {
  using System;
  using System.Drawing;

  public class Answer : IEquatable<Answer> {
    public bool correct;
    public string answer;
    [NonSerialized]
    public Image image;
    public string imageLoc;

    public bool Equals(Answer other) {
      if(correct == other.correct) {
        if(answer.Equals(other.answer)) {
          if((imageLoc == null && other.imageLoc == null) || imageLoc.Equals(other.imageLoc)) {
            if((image == null && other.image == null) || image.Equals(other.image)) {
              return true;
            }
          }
        }
      }
      return false;
    }
  }
}