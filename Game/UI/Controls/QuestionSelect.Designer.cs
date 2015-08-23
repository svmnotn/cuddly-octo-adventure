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
      this.structureTable = new System.Windows.Forms.TableLayoutPanel();
      this.teamLabel = new System.Windows.Forms.Label();
      this.structureTable.SuspendLayout();
      this.SuspendLayout();
      // 
      // questionsTable
      // 
      this.questionsTable.ColumnCount = 1;
      this.questionsTable.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
      this.questionsTable.Dock = System.Windows.Forms.DockStyle.Fill;
      this.questionsTable.Location = new System.Drawing.Point(3, 52);
      this.questionsTable.Name = "questionsTable";
      this.questionsTable.RowCount = 1;
      this.questionsTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 100F));
      this.questionsTable.Size = new System.Drawing.Size(484, 435);
      this.questionsTable.TabIndex = 0;
      // 
      // structureTable
      // 
      this.structureTable.ColumnCount = 1;
      this.structureTable.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
      this.structureTable.Controls.Add(this.teamLabel, 0, 0);
      this.structureTable.Controls.Add(this.questionsTable, 0, 1);
      this.structureTable.Dock = System.Windows.Forms.DockStyle.Fill;
      this.structureTable.Location = new System.Drawing.Point(5, 5);
      this.structureTable.Name = "structureTable";
      this.structureTable.RowCount = 2;
      this.structureTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 10F));
      this.structureTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 90F));
      this.structureTable.Size = new System.Drawing.Size(490, 490);
      this.structureTable.TabIndex = 0;
      // 
      // teamLabel
      // 
      this.teamLabel.AutoSize = true;
      this.teamLabel.Dock = System.Windows.Forms.DockStyle.Fill;
      this.teamLabel.Location = new System.Drawing.Point(3, 0);
      this.teamLabel.Name = "teamLabel";
      this.teamLabel.Size = new System.Drawing.Size(484, 49);
      this.teamLabel.TabIndex = 1;
      this.teamLabel.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
      // 
      // QuestionSelect
      // 
      this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
      this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
      this.Controls.Add(this.structureTable);
      this.Name = "QuestionSelect";
      this.Padding = new System.Windows.Forms.Padding(5);
      this.Size = new System.Drawing.Size(500, 500);
      this.structureTable.ResumeLayout(false);
      this.structureTable.PerformLayout();
      this.ResumeLayout(false);

    }

    #endregion

    private System.Windows.Forms.TableLayoutPanel questionsTable;
    private System.Windows.Forms.TableLayoutPanel structureTable;
    private System.Windows.Forms.Label teamLabel;
  }
}
