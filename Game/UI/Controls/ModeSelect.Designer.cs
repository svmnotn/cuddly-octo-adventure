namespace COA.Game.UI.Controls {
  partial class ModeSelect {
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
      this.components = new System.ComponentModel.Container();
      this.modeTable = new System.Windows.Forms.TableLayoutPanel();
      this.toolTip = new System.Windows.Forms.ToolTip(this.components);
      this.SuspendLayout();
      // 
      // modeTable
      // 
      this.modeTable.ColumnCount = 1;
      this.modeTable.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
      this.modeTable.Dock = System.Windows.Forms.DockStyle.Fill;
      this.modeTable.Location = new System.Drawing.Point(100, 100);
      this.modeTable.Name = "modeTable";
      this.modeTable.RowCount = 1;
      this.modeTable.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 100F));
      this.modeTable.Size = new System.Drawing.Size(300, 300);
      this.modeTable.TabIndex = 0;
      // 
      // ModeSelect
      // 
      this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
      this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
      this.Controls.Add(this.modeTable);
      this.Name = "ModeSelect";
      this.Padding = new System.Windows.Forms.Padding(100);
      this.Size = new System.Drawing.Size(500, 500);
      this.ResumeLayout(false);

    }

    #endregion

    private System.Windows.Forms.TableLayoutPanel modeTable;
    private System.Windows.Forms.ToolTip toolTip;
  }
}
