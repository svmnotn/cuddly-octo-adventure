namespace COA.Game.UI.Controls {
  using System;
  using System.Drawing;
  using System.Windows.Forms;

  internal class AnswerButton : Button {
    internal readonly bool correct;
    internal AnswerButton(bool correct) {
      this.correct = correct;
    }

    internal void Reveal() {
      Console.WriteLine(this + " is revealing that it is: " + correct);
      if(correct) {
        BackColor = Color.Green;
      } else {
        BackColor = Color.Red;
      }
    }
  }
}