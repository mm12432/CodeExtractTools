import { useState, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { Box, Button, TextField, Typography, Paper, Stack, Checkbox, IconButton, CircularProgress, Autocomplete, Chip, LinearProgress } from '@mui/material';
import DeleteIcon from '@mui/icons-material/Delete';
import FolderOpenIcon from '@mui/icons-material/FolderOpen';
import SearchIcon from '@mui/icons-material/Search';
import SyntaxHighlighter from 'react-syntax-highlighter';
import { vs2015 } from 'react-syntax-highlighter/dist/esm/styles/hljs';
import { FileTree, FileInfo } from './components/FileTree';

function App() {
  const [rootPath, setRootPath] = useState("");
  const [excludeRules, setExcludeRules] = useState<string[]>([".git", "target", "node_modules", "bin", "obj", ".vs", ".idea"]);
  const [newRule, setNewRule] = useState("");
  const [selectedExtensions, setSelectedExtensions] = useState<string[]>([]);

  const [files, setFiles] = useState<FileInfo[]>([]);
  const [selectedIds, setSelectedIds] = useState<Set<number>>(new Set());

  const [isScanning, setIsScanning] = useState(false);
  const [isExtracting, setIsExtracting] = useState(false);

  const [softwareName, setSoftwareName] = useState("");
  const [softwareVersion, setSoftwareVersion] = useState("V1.0");

  const [previewContent, setPreviewContent] = useState("请点击左侧开始扫描按钮，完毕后在目录树中勾选需要清洗的文件，然后点击本页右下角的 [执行深度清洗抽取] 等待结果...");
  const [previewLines, setPreviewLines] = useState(0);

  // 统计信息
  const selectedCount = selectedIds.size;
  const originalLines = useMemo(() => files.filter(f => selectedIds.has(f.id)).reduce((acc, f) => acc + f.lines, 0), [files, selectedIds]);
  // 简易预估页数（去除后台硬截取规则）
  const estimatedPages = Math.ceil(previewLines / 50);

  async function handleSelectDir() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });
      if (selected) {
        setRootPath(selected as string);
      }
    } catch (e) {
      console.error("Open dialog error:", e);
    }
  }

  async function handleScan() {
    if (!rootPath) {
      alert("请输入或使用文件夹图标选择扫描目录！");
      return;
    }
    setIsScanning(true);
    try {
      const result: any = await invoke("scan_project", {
        root: rootPath,
        customExcludes: excludeRules,
        extensions: selectedExtensions
      });
      // 更新文件树
      setFiles(result.files);

      // 动态将本次真实扫描到的所有代码后缀更新至界面的匹配列表中
      const foundExts = Object.keys(result.language_counts);
      setSelectedExtensions(foundExts);

      // 默认将扫描出来的文件全部设置为选中
      const allIds = new Set<number>(result.files.map((f: FileInfo) => f.id));
      setSelectedIds(allIds);
      setPreviewContent(`✅ 扫描结束！\n\n一共发现核心代码文件: ${result.total_files} 个。\n\n您可以在中间的「项目结构」树状图中展开各个层级，按需剔除不需要申报的文件（例如测试框架代码、编译配置脚本等）。\n\n配置完毕后，请点击右下角的执行深度提纯。`);
      setPreviewLines(0);
    } catch (e) {
      alert("扫描失败: " + e);
    } finally {
      setIsScanning(false);
    }
  }

  async function handleExtract() {
    const selectedFilePaths = files.filter(f => selectedIds.has(f.id)).map(f => f.absolute_path);
    if (selectedFilePaths.length === 0) {
      alert("请至少在结构树中勾选一个文件进行清洗！");
      return;
    }

    setIsExtracting(true);
    setPreviewContent("深度代码清洗执行中，请耐心等待...");
    try {
      const res: any = await invoke("execute_extraction", {
        files: selectedFilePaths,
        config: {
          remove_comments: true,
          remove_imports: true,
          compact_lines: true
        }
      });
      setPreviewContent(res.content);
      setPreviewLines(res.line_count);
    } catch (e) {
      alert("提取失败：" + e);
    } finally {
      setIsExtracting(false);
    }
  }

  function handleAddRule() {
    if (newRule && !excludeRules.includes(newRule)) {
      setExcludeRules([...excludeRules, newRule]);
      setNewRule("");
    }
  }

  function handleRemoveRule(index: number) {
    setExcludeRules(excludeRules.filter((_, i) => i !== index));
  }

  async function handleExportWord() {
    if (!previewContent) return;
    try {
      const savePath = await save({
        filters: [{
          name: 'Word 文档',
          extensions: ['docx']
        }],
        defaultPath: 'CodeExtractResult.docx',
      });

      if (savePath) {
        setIsExtracting(true); // 复用 extracting 状态展现加载等待图标
        const msg: string = await invoke("export_to_docx", {
          content: previewContent,
          config: {
            software_name: softwareName || "待修改软件名称",
            software_version: softwareVersion || "V1.0.0",
            save_path: savePath
          }
        });
        alert(msg);
      }
    } catch (e) {
      alert(`导出遇到错误: ${e}`);
    } finally {
      setIsExtracting(false);
    }
  }

  return (
    <Box sx={{ flexGrow: 1, height: '100vh', display: 'flex', flexDirection: 'column', bgcolor: '#f4f6f8' }}>

      {/* 顶部通栏 Dashboard */}
      <Paper elevation={1} sx={{ m: 2, p: 2, display: 'flex', justifyContent: 'space-around', alignItems: 'center', flexShrink: 0 }}>
        <Box textAlign="center">
          <Typography variant="caption" color="text.secondary">选中文件</Typography>
          <Typography variant="h5" fontWeight="bold" color="primary">{selectedCount} <Typography variant="caption" color="text.secondary">/ {files.length}</Typography></Typography>
        </Box>
        <Box textAlign="center">
          <Typography variant="caption" color="text.secondary">累计原始行数</Typography>
          <Typography variant="h5" fontWeight="bold">{originalLines}</Typography>
        </Box>
        <Box textAlign="center">
          <Typography variant="caption" color="text.secondary">深度清洗后净行数</Typography>
          <Typography variant="h5" fontWeight="bold" color="success.main">{previewLines}</Typography>
        </Box>
        <Box textAlign="center">
          <Typography variant="caption" color="text.secondary">预估 Word 页数 (按50行估)</Typography>
          <Typography variant="h5" fontWeight="bold" color="primary.main">{estimatedPages}</Typography>
        </Box>
      </Paper>

      {/* 预留高度的进度条包裹层，避免闪烁 */}
      <Box sx={{ height: 4, mx: 2, mb: 2 }}>
        {(isScanning || isExtracting) && (
          <LinearProgress sx={{ borderRadius: 2 }} />
        )}
      </Box>

      {/* 绝对控制的 Flex 极限四栏区域 */}
      <Box sx={{ display: 'flex', flexDirection: 'row', gap: 2, px: 2, pb: 2, flex: 1, overflow: 'hidden' }}>

        {/* 第1栏：配置与规则 (固定 280px) */}
        <Box sx={{ width: 280, display: 'flex', flexDirection: 'column', flexShrink: 0 }}>
          <Paper sx={{ p: 2, mb: 2 }}>
            <Typography variant="subtitle2" fontWeight="bold" mb={2}>1. 扫描配置</Typography>
            <Stack direction="row" spacing={1} mb={2}>
              <TextField
                size="small"
                placeholder="根目录绝对路径"
                value={rootPath}
                onChange={(e) => setRootPath(e.target.value)}
                fullWidth
              />
              <Button variant="outlined" sx={{ minWidth: '40px' }} onClick={handleSelectDir}>
                <FolderOpenIcon />
              </Button>
            </Stack>
            <Button
              variant="contained"
              startIcon={isScanning ? <CircularProgress size={20} color="inherit" /> : <SearchIcon />}
              fullWidth
              onClick={handleScan}
              disabled={isScanning}
            >
              {isScanning ? "目录扫描执行中..." : "👉 开始目录扫描"}
            </Button>
          </Paper>

          <Paper sx={{ p: 2, flexGrow: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
            <Typography variant="subtitle2" fontWeight="bold" mb={2}>2. 目标扩展名 (支持回车录入)</Typography>
            <Autocomplete
              multiple
              freeSolo
              size="small"
              options={[]}
              value={selectedExtensions}
              onChange={(_, newVal) => setSelectedExtensions(newVal)}
              renderTags={(value: readonly string[], getTagProps) =>
                value.map((option: string, index: number) => {
                  const { key, ...tagProps } = getTagProps({ index });
                  return <Chip variant="outlined" size="small" label={option} key={key} {...tagProps} />;
                })
              }
              renderInput={(params) => (
                <TextField {...params} placeholder="输入后缀如 js, rs..." />
              )}
              sx={{ mb: 3 }}
            />

            <Typography variant="subtitle2" fontWeight="bold" mb={2}>3. 排除匹配规则 (正则表达式)</Typography>
            <Stack direction="row" spacing={1} mb={2}>
              <TextField
                size="small"
                placeholder="正则表达式如 \.git$"
                value={newRule}
                onChange={(e) => setNewRule(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && handleAddRule()}
                fullWidth
              />
              <Button variant="contained" sx={{ minWidth: '40px', px: 1 }} onClick={handleAddRule}>┼</Button>
            </Stack>
            <Box sx={{ flexGrow: 1, overflowY: 'auto', pr: 1 }}>
              <Stack spacing={1}>
                {excludeRules.map((rule, idx) => (
                  <Paper key={idx} variant="outlined" sx={{ px: 1.5, py: 0.5, display: 'flex', alignItems: 'center', justifyContent: 'space-between', bgcolor: '#fafafa' }}>
                    <Typography variant="body2" sx={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                      {rule}
                    </Typography>
                    <IconButton size="small" onClick={() => handleRemoveRule(idx)}>
                      <DeleteIcon fontSize="small" color="action" />
                    </IconButton>
                  </Paper>
                ))}
              </Stack>
            </Box>
          </Paper>
        </Box>

        {/* 第2栏：项目结构树 (固定 320px) */}
        <Box sx={{ width: 320, display: 'flex', flexDirection: 'column', flexShrink: 0 }}>
          <Paper sx={{ p: 2, height: '100%', display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
            <Typography variant="subtitle2" fontWeight="bold" mb={2}>4. 项目结构 (联动勾选子文件)</Typography>
            <Box sx={{ flexGrow: 1, overflowX: 'auto', overflowY: 'auto', border: '1px solid #e0e0e0', borderRadius: 1 }}>
              <FileTree files={files} selectedIds={selectedIds} onSelectionChange={setSelectedIds} />
            </Box>
          </Paper>
        </Box>

        {/* 第3栏：预览大区 (自动伸缩拉满整个中部 flex: 1) */}
        <Box sx={{ flex: 1, display: 'flex', flexDirection: 'column', minWidth: 200 }}>
          <Paper sx={{ p: 2, height: '100%', display: 'flex', flexDirection: 'column' }}>
            <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 2 }}>
              <Typography variant="subtitle2" fontWeight="bold">5. 实时清洗代码大屏</Typography>
              <Button
                variant="contained"
                color="secondary"
                size="small"
                disabled={!previewContent || isExtracting}
                onClick={handleExportWord}
              >
                💾 一键生成软著标准文档 (.docx)
              </Button>
            </Box>
            <Box sx={{ flexGrow: 1, bgcolor: '#1e1e1e', color: '#d4d4d4', p: 2, borderRadius: 1, overflowY: 'auto', overflowX: 'hidden', fontSize: '13px' }}>
              {(() => {
                // 如果尚未抽取或只是提示信息，直接渲染普通文本
                if (!isExtracting && previewLines === 0) {
                  return <pre style={{ margin: 0, whiteSpace: 'pre-wrap', fontFamily: 'monospace' }}>{previewContent}</pre>;
                }

                // 否则说明有抽取的代码，我们根据 `// --- File: ` 作为分割符大块切分，逐个着色！
                const blocks = previewContent.split('// --- File: ');

                return blocks.map((block, idx) => {
                  if (!block.trim()) return null; // 排除空块

                  // 寻找这块的第一行（也就是刚才分割剩下的 "xxx.xxx ---" 加上换行）
                  const lineEndIdx = block.indexOf('\n');
                  if (lineEndIdx === -1) return <pre key={idx} style={{ margin: 0 }}>{block}</pre>;

                  const firstLine = block.substring(0, lineEndIdx);
                  const codeContent = block.substring(lineEndIdx + 1);

                  // 解析真实的文件扩展名
                  const extMatch = firstLine.match(/\.([^.\s-]+)\s---$/);
                  const ext = extMatch ? extMatch[1].toLowerCase() : 'txt';

                  // 将常见的扩展名映射到 Highlight.js 支持的语言名字
                  let syntaxLang = 'javascript'; // fallbacks
                  if (['ts', 'tsx'].includes(ext)) syntaxLang = 'typescript';
                  else if (['js', 'jsx'].includes(ext)) syntaxLang = 'javascript';
                  else if (['cs'].includes(ext)) syntaxLang = 'csharp';
                  else if (['rs'].includes(ext)) syntaxLang = 'rust';
                  else if (['py'].includes(ext)) syntaxLang = 'python';
                  else if (['html', 'vue'].includes(ext)) syntaxLang = 'xml'; // highlight.js 用 xml 代理
                  else if (['c', 'h'].includes(ext)) syntaxLang = 'c';
                  else if (['cpp', 'hpp', 'cc'].includes(ext)) syntaxLang = 'cpp';
                  else if (['go'].includes(ext)) syntaxLang = 'go';
                  else if (['java'].includes(ext)) syntaxLang = 'java';
                  else if (['css', 'less', 'scss'].includes(ext)) syntaxLang = 'css';

                  return (
                    <Box key={idx} sx={{ mb: 2 }}>
                      {/* 绿色刺眼的分割线档头 */}
                      <Typography sx={{ color: '#4EC9B0', fontWeight: 'bold', fontFamily: 'monospace', mb: 1 }}>
                        {`// --- File: ${firstLine}`}
                      </Typography>
                      {/* 真彩色代码渲染组件（采用 Highlight.js 引擎防止大型正则匹配断层） */}
                      <SyntaxHighlighter
                        language={syntaxLang}
                        style={vs2015}
                        customStyle={{ margin: 0, borderRadius: '4px', fontSize: '13px' }}
                      >
                        {codeContent}
                      </SyntaxHighlighter>
                    </Box>
                  );
                });
              })()}
            </Box>
          </Paper>
        </Box>

        {/* 第4栏：参数与操作 (固定 280px) */}
        <Box sx={{ width: 280, display: 'flex', flexDirection: 'column', flexShrink: 0 }}>
          <Paper sx={{ p: 2, flexGrow: 1, display: 'flex', flexDirection: 'column' }}>
            <Typography variant="subtitle2" fontWeight="bold" mb={2}>6. 深度清洗策略</Typography>
            <Stack spacing={0} mb={3}>
              <Stack direction="row" alignItems="center"><Checkbox size="small" defaultChecked /><Typography variant="body2">去除所有代码注释</Typography></Stack>
              <Stack direction="row" alignItems="center"><Checkbox size="small" defaultChecked /><Typography variant="body2">去除多余空行</Typography></Stack>
              <Stack direction="row" alignItems="center"><Checkbox size="small" defaultChecked /><Typography variant="body2">去空行及 import 声明</Typography></Stack>
            </Stack>

            <Typography variant="subtitle2" fontWeight="bold" mb={2}>7. 申报元数据</Typography>
            <Stack spacing={2} mb={3}>
              <TextField size="small" label="软件全称" placeholder="必填" value={softwareName} onChange={e => setSoftwareName(e.target.value)} fullWidth />
              <TextField size="small" label="版本号" value={softwareVersion} onChange={e => setSoftwareVersion(e.target.value)} fullWidth />
            </Stack>

            <Box mt="auto">
              <Button
                variant="contained"
                color="secondary"
                fullWidth
                sx={{ mb: 2, py: 1.5, fontWeight: 'bold' }}
                onClick={handleExtract}
                disabled={isExtracting}
              >
                {isExtracting ? "正在剔除杂项..." : "🚀 执行深度清洗提取"}
              </Button>
            </Box>
          </Paper>
        </Box>

      </Box>
    </Box>
  );
}

export default App;
