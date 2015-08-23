namespace COA.Game.UI.Controls {
  partial class QuestionPanel {
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
      this.structTable = new System.Windows.Forms.TableLayoutPanel();
      this.questionTable = new System.Windows.Forms.TableLayoutPanel();
      this.image = new System.Windows.Forms.PictureBox();
      this.text = new System.Windows.Forms.Label();
      this.answersTable = new System.Windows.Forms.TableLayoutPanel();
      this.timerLabel = new System.Windows.Forms.Label();
      this.infoTable = new System.Windows.Forms.TableLayoutPanel();
      this.teamLabel = new System.Windows.Forms.Label();
      this.structTable.SuspendLayout();
      this.questionTable.SuspendLayout();
      ((System.ComponentModel.ISupportInitialize)(this.image)).BeginInit();
      this.infoTable.SuspendLayout();
      this.SuspendLayout();
      // 
      // structTable
      // 
      this.structTable.ColumnCount = 1;
      this.structTable.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
      this.structTable.Controls.Add(this.infoTable, 0, 0);
      this.structTable.Controls.Add(this.questionTable, 0, 1);
      this.structTable.Controls.Add(this.answersTable, 0, 2);
      this.structTable.Dock = System.Windows.Forms.DockStyle.Fill;
      this.structTable.Location = new System.Drawing.Point(0, 0);
      this.structTable.Name = "structTable";
      this.structTable.RowCount = 3;
      this.structTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 10F));
      this.structTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 30F));
      this.structTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 60F));
      this.structTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
      this.structTable.Size = new System.Drawing.Size(500, 500);
      this.structTable.TabIndex = 0;
      // 
      // questionTable
      // 
      this.questionTable.ColumnCount = 2;
      this.questionTable.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 38.8664F));
      this.questionTable.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 61.1336F));
      this.questionTable.Controls.Add(this.image, 0, 0);
      this.questionTable.Controls.Add(this.text, 1, 0);
      this.questionTable.Dock = System.Windows.Forms.DockStyle.Fill;
      this.questionTable.Location = new System.Drawing.Point(3, 53);
      this.questionTable.Name = "questionTable";
      this.questionTable.RowCount = 1;
      this.questionTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50F));
      this.questionTable.Size = new System.Drawing.Size(494, 144);
      this.questionTable.TabIndex = 0;
      // 
      // image
      // 
      this.image.Dock = System.Windows.Forms.DockStyle.Fill;
      this.image.Location = new System.Drawing.Point(3, 3);
      this.image.Name = "image";
      this.image.Size = new System.Drawing.Size(186, 138);
      this.image.SizeMode = System.Windows.Forms.PictureBoxSizeMode.AutoSize;
      this.image.TabIndex = 0;
      this.image.TabStop = false;
      // 
      // text
      // 
      this.text.AutoSize = true;
      this.text.Dock = System.Windows.Forms.DockStyle.Fill;
      this.text.Location = new System.Drawing.Point(195, 0);
      this.text.Name = "text";
      this.text.Size = new System.Drawing.Size(296, 144);
      this.text.TabIndex = 1;
      // 
      // answersTable
      // 
      this.answersTable.AutoScroll = true;
      this.answersTable.ColumnCount = 1;
      this.answersTable.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
      this.answersTable.Dock = System.Windows.Forms.DockStyle.Fill;
      this.answersTable.Location = new System.Drawing.Point(3, 203);
      this.answersTable.Name = "answersTable";
      this.answersTable.RowCount = 1;
      this.answersTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50F));
      this.answersTable.Size = new System.Drawing.Size(494, 294);
      this.answersTable.TabIndex = 1;
      // 
      // timerLabel
      // 
      this.timerLabel.AutoSize = true;
      this.timerLabel.Dock = System.Windows.Forms.DockStyle.Fill;
      this.timerLabel.Location = new System.Drawing.Point(250, 0);
      this.timerLabel.Name = "timerLabel";
      this.timerLabel.Size = new System.Drawing.Size(241, 44);
      this.timerLabel.TabIndex = 2;
      this.timerLabel.Text = "Time Left:";
      this.timerLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
      // 
      // infoTable
      // 
      this.infoTable.ColumnCount = 2;
      this.infoTable.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
      this.infoTable.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
      this.infoTable.Controls.Add(this.teamLabel, 0, 0);
      this.infoTable.Controls.Add(this.timerLabel, 1, 0);
      this.infoTable.Dock = System.Windows.Forms.DockStyle.Fill;
      this.infoTable.Location = new System.Drawing.Point(3, 3);
      this.infoTable.Name = "infoTable";
      this.infoTable.RowCount = 1;
      this.infoTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50F));
      this.infoTable.Size = new System.Drawing.Size(494, 44);
      this.infoTable.TabIndex = 0;
      // 
      // teamLabel
      // 
      this.teamLabel.AutoSize = true;
      this.teamLabel.Dock = System.Windows.Forms.DockStyle.Fill;
      this.teamLabel.Location = new System.Drawing.Point(3, 0);
      this.teamLabel.Name = "teamLabel";
      this.teamLabel.Size = new System.Drawing.Size(241, 44);
      this.teamLabel.TabIndex = 3;
      this.teamLabel.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
      // 
      // Question
      // 
      this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
      this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
      this.Controls.Add(this.structTable);
      this.Name = "Question";
      this.Size = new System.Drawing.Size(500, 500);
      this.structTable.ResumeLayout(false);
      this.questionTable.ResumeLayout(false);
      this.questionTable.PerformLayout();
      ((System.ComponentModel.ISupportInitialize)(this.image)).EndInit();
      this.infoTable.ResumeLayout(false);
      this.infoTable.PerformLayout();
      this.ResumeLayout(false);

    }

    #endregion

    private System.Windows.Forms.TableLayoutPanel structTable;
    private System.Windows.Forms.TableLayoutPanel questionTable;
    private System.Windows.Forms.PictureBox image;
    private System.Windows.Forms.Label text;
    private System.Windows.Forms.TableLayoutPanel answersTable;
    private System.Windows.Forms.Label timerLabel;
    private System.Windows.Forms.TableLayoutPanel infoTable;
    private System.Windows.Forms.Label teamLabel;
  }
}
