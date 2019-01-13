using System;
using System.Collections.Generic;
using System.IO;
using System.Data;
using System.Linq;
using System.Text;
using System.Windows.Forms;
using System.Text.RegularExpressions;

namespace CodeExtractTools
{
    public partial class FormMain : Form
    {
        public FormMain()
        {
            InitializeComponent();
        }

        private int sourceCodeLines;
        private StringBuilder stringBuilder;

        private void buttonSelect_Click(object sender, EventArgs e)
        {
            FolderBrowserDialog folderBrowserDialog = new FolderBrowserDialog();
            if (folderBrowserDialog.ShowDialog() == DialogResult.OK)
            {
                this.textBoxPath.Text = folderBrowserDialog.SelectedPath;
            }
        }

        private void buttonStart_Click(object sender, EventArgs e)
        {
            string pathRoot = textBoxPath.Text;
            if (!string.IsNullOrEmpty(pathRoot))
            {
                sourceCodeLines = 0;
                stringBuilder = new StringBuilder();
                Encoding encodingSrc = getSourceEncoding();
                string[] filters = this.richTextBoxFilter.Lines.Where(line => !string.IsNullOrEmpty(line)).ToArray();
                string[] includes = this.richTextBoxInclude.Lines.Where(line => !string.IsNullOrEmpty(line)).ToArray();
                DirectoryInfo directoryInfo = new DirectoryInfo(pathRoot);
                RecursionDir(directoryInfo, filters, includes, encodingSrc);
                this.listBoxOutput.Items.Add(string.Format("成功读取代码{0}行", sourceCodeLines));
            }
        }

        private void RecursionDir(DirectoryInfo parent, string[] filters, string[] includes, Encoding encoding)
        {
            DirectoryInfo[] directoryInfoChildren = parent.GetDirectories();
            FileInfo[] fileInfos = parent.GetFiles();
            ProcessFiles(filters, includes, fileInfos, encoding);
            foreach (var dir in directoryInfoChildren)
            {
                bool filterd = false;
                foreach (var filter in filters)
                {
                    Regex regex = new Regex(@filter);
                    if (regex.IsMatch(dir.Name))
                    {
                        filterd = true;
                        break;
                    }
                }
                if (!filterd)
                {
                    RecursionDir(dir, filters, includes, encoding);
                }
            }
        }

        private void ProcessFiles(string[] filters, string[] includes, FileInfo[] fileInfos, Encoding encoding)
        {
            foreach (var file in fileInfos)
            {
                bool filterd = false;
                bool included = false;
                foreach (var filter in filters)
                {
                    Regex regex = new Regex(@filter);
                    if (regex.IsMatch(file.Name))
                    {
                        filterd = true;
                        break;
                    }
                }
                foreach (var include in includes)
                {
                    if (file.Extension == include)
                    {
                        included = true;
                        break;
                    }
                }
                if (!filterd && included)
                {
                    LoadFile(file.FullName, encoding);
                }
            }
        }

        private void LoadFile(string fileName, Encoding encoding)
        {
            using (FileStream fileStream = new FileStream(fileName, FileMode.Open))
            {
                using (StreamReader streamReader = new StreamReader(fileStream, encoding))
                {
                    string s = streamReader.ReadLine();
                    while (s != null)
                    {
                        sourceCodeLines++;
                        this.stringBuilder.AppendLine(s);
                        //this.listBoxOutput.Items.Add(s);
                        s = streamReader.ReadLine();
                    }
                    streamReader.Close();
                }
                fileStream.Close();
            }
        }

        private void buttonSave_Click(object sender, EventArgs e)
        {
            SaveFileDialog saveFileDialog = new SaveFileDialog();
            saveFileDialog.Filter = "所有文件|*.*";
            if (saveFileDialog.ShowDialog() == DialogResult.OK)
            {
                Encoding encodingTar = getTargetEncoding();
                using (FileStream fileStream = new FileStream(saveFileDialog.FileName, FileMode.Create))
                {
                    using (StreamWriter streamWriter = new StreamWriter(fileStream, encodingTar))
                    {
                        streamWriter.Write(stringBuilder.ToString());
                        streamWriter.Flush();
                        streamWriter.Close();
                    }
                    fileStream.Close();
                }
                this.listBoxOutput.Items.Add(string.Format("文件{0}保存成功", saveFileDialog.FileName));
            }
        }

        private Encoding getSourceEncoding()
        {
            if (radioButtonSourceDefault.Checked)
            {
                return Encoding.Default;
            }
            else if (radioButtonSourceUTF8.Checked)
            {
                return Encoding.UTF8;
            }
            else
            {
                return Encoding.Unicode;
            }
        }

        private Encoding getTargetEncoding()
        {
            if (radioButtonTargetDefault.Checked)
            {
                return Encoding.Default;
            }
            else if (radioButtonTargetUTF8.Checked)
            {
                return Encoding.UTF8;
            }
            else
            {
                return Encoding.Unicode;
            }
        }
    }
}
