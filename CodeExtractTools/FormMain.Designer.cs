namespace CodeExtractTools
{
    partial class FormMain
    {
        /// <summary>
        /// 必需的设计器变量。
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// 清理所有正在使用的资源。
        /// </summary>
        /// <param name="disposing">如果应释放托管资源，为 true；否则为 false。</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows 窗体设计器生成的代码

        /// <summary>
        /// 设计器支持所需的方法 - 不要修改
        /// 使用代码编辑器修改此方法的内容。
        /// </summary>
        private void InitializeComponent()
        {
            this.tableLayoutPanelMain = new System.Windows.Forms.TableLayoutPanel();
            this.groupBoxFilter = new System.Windows.Forms.GroupBox();
            this.groupBoxOperation = new System.Windows.Forms.GroupBox();
            this.groupBoxInclude = new System.Windows.Forms.GroupBox();
            this.groupBoxOutput = new System.Windows.Forms.GroupBox();
            this.listBoxOutput = new System.Windows.Forms.ListBox();
            this.richTextBoxFilter = new System.Windows.Forms.RichTextBox();
            this.richTextBoxInclude = new System.Windows.Forms.RichTextBox();
            this.buttonSelect = new System.Windows.Forms.Button();
            this.textBoxPath = new System.Windows.Forms.TextBox();
            this.buttonStart = new System.Windows.Forms.Button();
            this.buttonSave = new System.Windows.Forms.Button();
            this.groupBoxSource = new System.Windows.Forms.GroupBox();
            this.groupBoxTarget = new System.Windows.Forms.GroupBox();
            this.radioButtonSourceDefault = new System.Windows.Forms.RadioButton();
            this.radioButtonSourceUTF8 = new System.Windows.Forms.RadioButton();
            this.radioButtonSourceUnicode = new System.Windows.Forms.RadioButton();
            this.radioButtonTargetDefault = new System.Windows.Forms.RadioButton();
            this.radioButtonTargetUTF8 = new System.Windows.Forms.RadioButton();
            this.radioButtonTargetUnicode = new System.Windows.Forms.RadioButton();
            this.tableLayoutPanelMain.SuspendLayout();
            this.groupBoxFilter.SuspendLayout();
            this.groupBoxOperation.SuspendLayout();
            this.groupBoxInclude.SuspendLayout();
            this.groupBoxOutput.SuspendLayout();
            this.groupBoxSource.SuspendLayout();
            this.groupBoxTarget.SuspendLayout();
            this.SuspendLayout();
            // 
            // tableLayoutPanelMain
            // 
            this.tableLayoutPanelMain.ColumnCount = 2;
            this.tableLayoutPanelMain.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
            this.tableLayoutPanelMain.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
            this.tableLayoutPanelMain.Controls.Add(this.groupBoxFilter, 0, 0);
            this.tableLayoutPanelMain.Controls.Add(this.groupBoxOperation, 1, 0);
            this.tableLayoutPanelMain.Controls.Add(this.groupBoxInclude, 0, 1);
            this.tableLayoutPanelMain.Controls.Add(this.groupBoxOutput, 1, 1);
            this.tableLayoutPanelMain.Dock = System.Windows.Forms.DockStyle.Fill;
            this.tableLayoutPanelMain.Location = new System.Drawing.Point(0, 0);
            this.tableLayoutPanelMain.Name = "tableLayoutPanelMain";
            this.tableLayoutPanelMain.RowCount = 2;
            this.tableLayoutPanelMain.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50F));
            this.tableLayoutPanelMain.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50F));
            this.tableLayoutPanelMain.Size = new System.Drawing.Size(784, 562);
            this.tableLayoutPanelMain.TabIndex = 0;
            // 
            // groupBoxFilter
            // 
            this.groupBoxFilter.Controls.Add(this.richTextBoxFilter);
            this.groupBoxFilter.Dock = System.Windows.Forms.DockStyle.Fill;
            this.groupBoxFilter.Location = new System.Drawing.Point(3, 3);
            this.groupBoxFilter.Name = "groupBoxFilter";
            this.groupBoxFilter.Size = new System.Drawing.Size(386, 275);
            this.groupBoxFilter.TabIndex = 0;
            this.groupBoxFilter.TabStop = false;
            this.groupBoxFilter.Text = "代码文件名过滤规则(支持正则表达式-对文件夹有效)";
            // 
            // groupBoxOperation
            // 
            this.groupBoxOperation.Controls.Add(this.groupBoxTarget);
            this.groupBoxOperation.Controls.Add(this.groupBoxSource);
            this.groupBoxOperation.Controls.Add(this.buttonSave);
            this.groupBoxOperation.Controls.Add(this.buttonStart);
            this.groupBoxOperation.Controls.Add(this.textBoxPath);
            this.groupBoxOperation.Controls.Add(this.buttonSelect);
            this.groupBoxOperation.Dock = System.Windows.Forms.DockStyle.Fill;
            this.groupBoxOperation.Location = new System.Drawing.Point(395, 3);
            this.groupBoxOperation.Name = "groupBoxOperation";
            this.groupBoxOperation.Size = new System.Drawing.Size(386, 275);
            this.groupBoxOperation.TabIndex = 1;
            this.groupBoxOperation.TabStop = false;
            this.groupBoxOperation.Text = "操作和选项";
            // 
            // groupBoxInclude
            // 
            this.groupBoxInclude.Controls.Add(this.richTextBoxInclude);
            this.groupBoxInclude.Dock = System.Windows.Forms.DockStyle.Fill;
            this.groupBoxInclude.Location = new System.Drawing.Point(3, 284);
            this.groupBoxInclude.Name = "groupBoxInclude";
            this.groupBoxInclude.Size = new System.Drawing.Size(386, 275);
            this.groupBoxInclude.TabIndex = 2;
            this.groupBoxInclude.TabStop = false;
            this.groupBoxInclude.Text = "代码文件名包含规则(仅对文件有效)";
            // 
            // groupBoxOutput
            // 
            this.groupBoxOutput.Controls.Add(this.listBoxOutput);
            this.groupBoxOutput.Dock = System.Windows.Forms.DockStyle.Fill;
            this.groupBoxOutput.Location = new System.Drawing.Point(395, 284);
            this.groupBoxOutput.Name = "groupBoxOutput";
            this.groupBoxOutput.Size = new System.Drawing.Size(386, 275);
            this.groupBoxOutput.TabIndex = 3;
            this.groupBoxOutput.TabStop = false;
            this.groupBoxOutput.Text = "输出";
            // 
            // listBoxOutput
            // 
            this.listBoxOutput.Dock = System.Windows.Forms.DockStyle.Fill;
            this.listBoxOutput.FormattingEnabled = true;
            this.listBoxOutput.ItemHeight = 12;
            this.listBoxOutput.Location = new System.Drawing.Point(3, 17);
            this.listBoxOutput.Name = "listBoxOutput";
            this.listBoxOutput.Size = new System.Drawing.Size(380, 255);
            this.listBoxOutput.TabIndex = 0;
            // 
            // richTextBoxFilter
            // 
            this.richTextBoxFilter.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
            this.richTextBoxFilter.Dock = System.Windows.Forms.DockStyle.Fill;
            this.richTextBoxFilter.Location = new System.Drawing.Point(3, 17);
            this.richTextBoxFilter.Name = "richTextBoxFilter";
            this.richTextBoxFilter.Size = new System.Drawing.Size(380, 255);
            this.richTextBoxFilter.TabIndex = 0;
            this.richTextBoxFilter.Text = "";
            // 
            // richTextBoxInclude
            // 
            this.richTextBoxInclude.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
            this.richTextBoxInclude.Dock = System.Windows.Forms.DockStyle.Fill;
            this.richTextBoxInclude.Location = new System.Drawing.Point(3, 17);
            this.richTextBoxInclude.Name = "richTextBoxInclude";
            this.richTextBoxInclude.Size = new System.Drawing.Size(380, 255);
            this.richTextBoxInclude.TabIndex = 0;
            this.richTextBoxInclude.Text = "";
            // 
            // buttonSelect
            // 
            this.buttonSelect.Location = new System.Drawing.Point(6, 48);
            this.buttonSelect.Name = "buttonSelect";
            this.buttonSelect.Size = new System.Drawing.Size(75, 23);
            this.buttonSelect.TabIndex = 1;
            this.buttonSelect.Text = "请选择目录";
            this.buttonSelect.UseVisualStyleBackColor = true;
            this.buttonSelect.Click += new System.EventHandler(this.buttonSelect_Click);
            // 
            // textBoxPath
            // 
            this.textBoxPath.Location = new System.Drawing.Point(7, 21);
            this.textBoxPath.Name = "textBoxPath";
            this.textBoxPath.ReadOnly = true;
            this.textBoxPath.Size = new System.Drawing.Size(373, 21);
            this.textBoxPath.TabIndex = 2;
            // 
            // buttonStart
            // 
            this.buttonStart.Location = new System.Drawing.Point(223, 246);
            this.buttonStart.Name = "buttonStart";
            this.buttonStart.Size = new System.Drawing.Size(75, 23);
            this.buttonStart.TabIndex = 3;
            this.buttonStart.Text = "开始提取";
            this.buttonStart.UseVisualStyleBackColor = true;
            this.buttonStart.Click += new System.EventHandler(this.buttonStart_Click);
            // 
            // buttonSave
            // 
            this.buttonSave.Location = new System.Drawing.Point(305, 246);
            this.buttonSave.Name = "buttonSave";
            this.buttonSave.Size = new System.Drawing.Size(75, 23);
            this.buttonSave.TabIndex = 4;
            this.buttonSave.Text = "保存";
            this.buttonSave.UseVisualStyleBackColor = true;
            this.buttonSave.Click += new System.EventHandler(this.buttonSave_Click);
            // 
            // groupBoxSource
            // 
            this.groupBoxSource.Controls.Add(this.radioButtonSourceUnicode);
            this.groupBoxSource.Controls.Add(this.radioButtonSourceUTF8);
            this.groupBoxSource.Controls.Add(this.radioButtonSourceDefault);
            this.groupBoxSource.Location = new System.Drawing.Point(7, 78);
            this.groupBoxSource.Name = "groupBoxSource";
            this.groupBoxSource.Size = new System.Drawing.Size(170, 100);
            this.groupBoxSource.TabIndex = 5;
            this.groupBoxSource.TabStop = false;
            this.groupBoxSource.Text = "源文件编码";
            // 
            // groupBoxTarget
            // 
            this.groupBoxTarget.Controls.Add(this.radioButtonTargetUnicode);
            this.groupBoxTarget.Controls.Add(this.radioButtonTargetUTF8);
            this.groupBoxTarget.Controls.Add(this.radioButtonTargetDefault);
            this.groupBoxTarget.Location = new System.Drawing.Point(210, 78);
            this.groupBoxTarget.Name = "groupBoxTarget";
            this.groupBoxTarget.Size = new System.Drawing.Size(170, 100);
            this.groupBoxTarget.TabIndex = 6;
            this.groupBoxTarget.TabStop = false;
            this.groupBoxTarget.Text = "目标文件编码";
            // 
            // radioButtonSourceDefault
            // 
            this.radioButtonSourceDefault.AutoSize = true;
            this.radioButtonSourceDefault.Checked = true;
            this.radioButtonSourceDefault.Location = new System.Drawing.Point(7, 20);
            this.radioButtonSourceDefault.Name = "radioButtonSourceDefault";
            this.radioButtonSourceDefault.Size = new System.Drawing.Size(71, 16);
            this.radioButtonSourceDefault.TabIndex = 0;
            this.radioButtonSourceDefault.TabStop = true;
            this.radioButtonSourceDefault.Text = "系统默认";
            this.radioButtonSourceDefault.UseVisualStyleBackColor = true;
            // 
            // radioButtonSourceUTF8
            // 
            this.radioButtonSourceUTF8.AutoSize = true;
            this.radioButtonSourceUTF8.Location = new System.Drawing.Point(6, 42);
            this.radioButtonSourceUTF8.Name = "radioButtonSourceUTF8";
            this.radioButtonSourceUTF8.Size = new System.Drawing.Size(53, 16);
            this.radioButtonSourceUTF8.TabIndex = 1;
            this.radioButtonSourceUTF8.Text = "UTF-8";
            this.radioButtonSourceUTF8.UseVisualStyleBackColor = true;
            // 
            // radioButtonSourceUnicode
            // 
            this.radioButtonSourceUnicode.AutoSize = true;
            this.radioButtonSourceUnicode.Location = new System.Drawing.Point(6, 65);
            this.radioButtonSourceUnicode.Name = "radioButtonSourceUnicode";
            this.radioButtonSourceUnicode.Size = new System.Drawing.Size(65, 16);
            this.radioButtonSourceUnicode.TabIndex = 2;
            this.radioButtonSourceUnicode.TabStop = true;
            this.radioButtonSourceUnicode.Text = "Unicode";
            this.radioButtonSourceUnicode.UseVisualStyleBackColor = true;
            // 
            // radioButtonTargetDefault
            // 
            this.radioButtonTargetDefault.AutoSize = true;
            this.radioButtonTargetDefault.Checked = true;
            this.radioButtonTargetDefault.Location = new System.Drawing.Point(7, 19);
            this.radioButtonTargetDefault.Name = "radioButtonTargetDefault";
            this.radioButtonTargetDefault.Size = new System.Drawing.Size(71, 16);
            this.radioButtonTargetDefault.TabIndex = 0;
            this.radioButtonTargetDefault.TabStop = true;
            this.radioButtonTargetDefault.Text = "系统默认";
            this.radioButtonTargetDefault.UseVisualStyleBackColor = true;
            // 
            // radioButtonTargetUTF8
            // 
            this.radioButtonTargetUTF8.AutoSize = true;
            this.radioButtonTargetUTF8.Location = new System.Drawing.Point(7, 41);
            this.radioButtonTargetUTF8.Name = "radioButtonTargetUTF8";
            this.radioButtonTargetUTF8.Size = new System.Drawing.Size(53, 16);
            this.radioButtonTargetUTF8.TabIndex = 1;
            this.radioButtonTargetUTF8.TabStop = true;
            this.radioButtonTargetUTF8.Text = "UTF-8";
            this.radioButtonTargetUTF8.UseVisualStyleBackColor = true;
            // 
            // radioButtonTargetUnicode
            // 
            this.radioButtonTargetUnicode.AutoSize = true;
            this.radioButtonTargetUnicode.Location = new System.Drawing.Point(7, 64);
            this.radioButtonTargetUnicode.Name = "radioButtonTargetUnicode";
            this.radioButtonTargetUnicode.Size = new System.Drawing.Size(65, 16);
            this.radioButtonTargetUnicode.TabIndex = 2;
            this.radioButtonTargetUnicode.TabStop = true;
            this.radioButtonTargetUnicode.Text = "Unicode";
            this.radioButtonTargetUnicode.UseVisualStyleBackColor = true;
            // 
            // FormMain
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 12F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(784, 562);
            this.Controls.Add(this.tableLayoutPanelMain);
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedSingle;
            this.MaximizeBox = false;
            this.MinimizeBox = false;
            this.Name = "FormMain";
            this.Text = "中国软件著作权源代码提取工具";
            this.tableLayoutPanelMain.ResumeLayout(false);
            this.groupBoxFilter.ResumeLayout(false);
            this.groupBoxOperation.ResumeLayout(false);
            this.groupBoxOperation.PerformLayout();
            this.groupBoxInclude.ResumeLayout(false);
            this.groupBoxOutput.ResumeLayout(false);
            this.groupBoxSource.ResumeLayout(false);
            this.groupBoxSource.PerformLayout();
            this.groupBoxTarget.ResumeLayout(false);
            this.groupBoxTarget.PerformLayout();
            this.ResumeLayout(false);

        }

        #endregion

        private System.Windows.Forms.TableLayoutPanel tableLayoutPanelMain;
        private System.Windows.Forms.GroupBox groupBoxFilter;
        private System.Windows.Forms.GroupBox groupBoxOperation;
        private System.Windows.Forms.GroupBox groupBoxInclude;
        private System.Windows.Forms.GroupBox groupBoxOutput;
        private System.Windows.Forms.ListBox listBoxOutput;
        private System.Windows.Forms.RichTextBox richTextBoxFilter;
        private System.Windows.Forms.RichTextBox richTextBoxInclude;
        private System.Windows.Forms.Button buttonSave;
        private System.Windows.Forms.Button buttonStart;
        private System.Windows.Forms.TextBox textBoxPath;
        private System.Windows.Forms.Button buttonSelect;
        private System.Windows.Forms.GroupBox groupBoxSource;
        private System.Windows.Forms.GroupBox groupBoxTarget;
        private System.Windows.Forms.RadioButton radioButtonTargetUnicode;
        private System.Windows.Forms.RadioButton radioButtonTargetUTF8;
        private System.Windows.Forms.RadioButton radioButtonTargetDefault;
        private System.Windows.Forms.RadioButton radioButtonSourceUnicode;
        private System.Windows.Forms.RadioButton radioButtonSourceUTF8;
        private System.Windows.Forms.RadioButton radioButtonSourceDefault;
    }
}

