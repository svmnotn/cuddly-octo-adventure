namespace COA.Data {
  using System;
  using System.Drawing;
  using System.Collections.Generic;

  public class Question : IEquatable<Question> {
    public string id;
    public int value;
    public string question;
    [NonSerialized]
    public Image image;
    public string imageLoc;
    public List<Answer> answers;

    public bool Equals(Question other) {
      if(id != null && other.id != null && id.Equals(other.id)) {// ID Can't be null
        if(value == other.value) {
          if((question == null && other.question == null) || question.Equals(other.question)) {
            if((imageLoc == null && other.imageLoc == null) || imageLoc.Equals(other.imageLoc)) {
              if((image == null && other.image == null) || image.Equals(other.image)) {
                if(answers != null && other.answers != null && answers.UnorderedEqual(other.answers)) {
                  return true;
                }
              }
            }
          }
        }
      }
      return false;
    }
  }
}