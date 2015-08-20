namespace COA.Game.UI.Controls {
  using System.Windows.Forms;

  internal partial class Question : UserControl {
    Data.Question question;
    internal Question(Data.Question q) {
      InitializeComponent();
      question = q;
    }
  }
}
