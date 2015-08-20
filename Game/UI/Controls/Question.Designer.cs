namespace COA.Game.UI.Controls {
  partial class Question {
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
      this.structTable.SuspendLayout();
      this.questionTable.SuspendLayout();
      ((System.ComponentModel.ISupportInitialize)(this.image)).BeginInit();
      this.SuspendLayout();
      // 
      // structTable
      // 
      this.structTable.ColumnCount = 1;
      this.structTable.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
      this.structTable.Controls.Add(this.questionTable, 0, 0);
      this.structTable.Controls.Add(this.answersTable, 0, 1);
      this.structTable.Dock = System.Windows.Forms.DockStyle.Fill;
      this.structTable.Location = new System.Drawing.Point(0, 0);
      this.structTable.Name = "structTable";
      this.structTable.RowCount = 2;
      this.structTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 38F));
      this.structTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 62F));
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
      this.questionTable.Location = new System.Drawing.Point(3, 3);
      this.questionTable.Name = "questionTable";
      this.questionTable.RowCount = 1;
      this.questionTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50F));
      this.questionTable.Size = new System.Drawing.Size(494, 184);
      this.questionTable.TabIndex = 0;
      // 
      // image
      // 
      this.image.Dock = System.Windows.Forms.DockStyle.Fill;
      this.image.Location = new System.Drawing.Point(3, 3);
      this.image.Name = "image";
      this.image.Size = new System.Drawing.Size(186, 178);
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
      this.text.Size = new System.Drawing.Size(296, 184);
      this.text.TabIndex = 1;
      this.text.Text = "label1";
      // 
      // answersTable
      // 
      this.answersTable.ColumnCount = 1;
      this.answersTable.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
      this.answersTable.Dock = System.Windows.Forms.DockStyle.Fill;
      this.answersTable.Location = new System.Drawing.Point(3, 193);
      this.answersTable.Name = "answersTable";
      this.answersTable.RowCount = 1;
      this.answersTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50F));
      this.answersTable.Size = new System.Drawing.Size(494, 304);
      this.answersTable.TabIndex = 1;
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
      this.ResumeLayout(false);

    }

    #endregion

    private System.Windows.Forms.TableLayoutPanel structTable;
    private System.Windows.Forms.TableLayoutPanel questionTable;
    private System.Windows.Forms.PictureBox image;
    private System.Windows.Forms.Label text;
    private System.Windows.Forms.TableLayoutPanel answersTable;
  }
}
