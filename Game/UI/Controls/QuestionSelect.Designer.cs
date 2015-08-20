namespace COA.Game.UI.Controls {
  partial class QuestionSelect {
    /// <summary> 
    /// Required designer variable.
    /// </summary>
    private System.ComponentModel.IContainer components = null;

    /// <summary> 
    /// Clean up any resources being used.
    /// </summary>
    /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
    protected override void Dispose(bool disposing) {
      if(disposing && (components != null)) {
        components.Dispose();
      }
      base.Dispose(disposing);
    }

    #region Component Designer generated code

    /// <summary> 
    /// Required method for Designer support - do not modify 
    /// the contents of this method with the code editor.
    /// </summary>
    private void InitializeComponent() {
      this.questionsTable = new System.Windows.Forms.TableLayoutPanel();
      this.SuspendLayout();
      // 
      // questionsTable
      // 
      this.questionsTable.ColumnCount = 1;
      this.questionsTable.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
      this.questionsTable.Dock = System.Windows.Forms.DockStyle.Fill;
      this.questionsTable.Location = new System.Drawing.Point(5, 5);
      this.questionsTable.Name = "questionsTable";
      this.questionsTable.RowCount = 1;
      this.questionsTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 100F));
      this.questionsTable.Size = new System.Drawing.Size(490, 490);
      this.questionsTable.TabIndex = 0;
      // 
      // QuestionSelect
      // 
      this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
      this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
      this.Controls.Add(this.questionsTable);
      this.Name = "QuestionSelect";
      this.Padding = new System.Windows.Forms.Padding(5);
      this.Size = new System.Drawing.Size(500, 500);
      this.ResumeLayout(false);

    }

    #endregion

    private System.Windows.Forms.TableLayoutPanel questionsTable;
  }
}
