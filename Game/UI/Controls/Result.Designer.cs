namespace COA.Game.UI.Controls {
  partial class Result {
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
      this.structureTable = new System.Windows.Forms.TableLayoutPanel();
      this.label = new System.Windows.Forms.Label();
      this.table = new System.Windows.Forms.TableLayoutPanel();
      this.teamsLabel = new System.Windows.Forms.Label();
      this.scoresLabel = new System.Windows.Forms.Label();
      this.structureTable.SuspendLayout();
      this.table.SuspendLayout();
      this.SuspendLayout();
      // 
      // structureTable
      // 
      this.structureTable.ColumnCount = 1;
      this.structureTable.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
      this.structureTable.Controls.Add(this.label, 0, 0);
      this.structureTable.Controls.Add(this.table, 0, 1);
      this.structureTable.Dock = System.Windows.Forms.DockStyle.Fill;
      this.structureTable.Location = new System.Drawing.Point(0, 0);
      this.structureTable.Name = "structureTable";
      this.structureTable.RowCount = 2;
      this.structureTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 12F));
      this.structureTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 88F));
      this.structureTable.Size = new System.Drawing.Size(500, 500);
      this.structureTable.TabIndex = 0;
      // 
      // label
      // 
      this.label.AutoSize = true;
      this.label.Dock = System.Windows.Forms.DockStyle.Fill;
      this.label.Location = new System.Drawing.Point(3, 0);
      this.label.Name = "label";
      this.label.Size = new System.Drawing.Size(494, 60);
      this.label.TabIndex = 0;
      this.label.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
      // 
      // table
      // 
      this.table.ColumnCount = 2;
      this.table.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
      this.table.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
      this.table.Controls.Add(this.teamsLabel, 0, 0);
      this.table.Controls.Add(this.scoresLabel, 1, 0);
      this.table.Dock = System.Windows.Forms.DockStyle.Fill;
      this.table.Location = new System.Drawing.Point(3, 63);
      this.table.Name = "table";
      this.table.RowCount = 2;
      this.table.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 5F));
      this.table.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 95F));
      this.table.Size = new System.Drawing.Size(494, 434);
      this.table.TabIndex = 1;
      // 
      // teamsLabel
      // 
      this.teamsLabel.AutoSize = true;
      this.teamsLabel.Dock = System.Windows.Forms.DockStyle.Fill;
      this.teamsLabel.Location = new System.Drawing.Point(3, 0);
      this.teamsLabel.Name = "teamsLabel";
      this.teamsLabel.Size = new System.Drawing.Size(241, 21);
      this.teamsLabel.TabIndex = 0;
      this.teamsLabel.Text = "Teams";
      this.teamsLabel.TextAlign = System.Drawing.ContentAlignment.TopCenter;
      // 
      // scoresLabel
      // 
      this.scoresLabel.AutoSize = true;
      this.scoresLabel.Dock = System.Windows.Forms.DockStyle.Fill;
      this.scoresLabel.Location = new System.Drawing.Point(250, 0);
      this.scoresLabel.Name = "scoresLabel";
      this.scoresLabel.Size = new System.Drawing.Size(241, 21);
      this.scoresLabel.TabIndex = 1;
      this.scoresLabel.Text = "Score";
      this.scoresLabel.TextAlign = System.Drawing.ContentAlignment.TopCenter;
      // 
      // Result
      // 
      this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
      this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
      this.Controls.Add(this.structureTable);
      this.Name = "Result";
      this.Size = new System.Drawing.Size(500, 500);
      this.structureTable.ResumeLayout(false);
      this.structureTable.PerformLayout();
      this.table.ResumeLayout(false);
      this.table.PerformLayout();
      this.ResumeLayout(false);

    }

    #endregion

    private System.Windows.Forms.TableLayoutPanel structureTable;
    private System.Windows.Forms.Label label;
    private System.Windows.Forms.TableLayoutPanel table;
    private System.Windows.Forms.Label teamsLabel;
    private System.Windows.Forms.Label scoresLabel;
  }
}
