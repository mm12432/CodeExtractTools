import React, { useState, useMemo } from 'react';
import { List, ListItem, ListItemIcon, ListItemText, Checkbox, Collapse, IconButton } from '@mui/material';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import ChevronRightIcon from '@mui/icons-material/ChevronRight';
import InsertDriveFileOutlinedIcon from '@mui/icons-material/InsertDriveFileOutlined';
import FolderOutlinedIcon from '@mui/icons-material/FolderOutlined';

export interface FileInfo {
    id: number;
    absolute_path: string;
    relative_path: string;
    lines: number;
    extension: string;
}

// 帮助函数：基于扩展名返回特定的主题色彩
const getFileIconColor = (ext?: string): string => {
    switch (ext?.toLowerCase()) {
        case 'ts': case 'tsx': return '#3178c6'; // TypeScript Blue
        case 'js': case 'jsx': return '#f7df1e'; // JS Yellow
        case 'vue': return '#41b883'; // Vue Green
        case 'rs': return '#dea584'; // Rust Orange/Brown
        case 'py': return '#3572A5'; // Python Blue
        case 'html': return '#e34c26'; // HTML Red
        case 'css': case 'scss': case 'less': return '#563d7c'; // CSS Purple
        case 'json': return '#292929';
        case 'cpp': case 'c': case 'h': case 'hpp': return '#f34b7d'; // C++ Pink/Red
        case 'cs': return '#178600'; // C# Green
        case 'java': return '#b07219'; // Java Brown
        case 'go': return '#00ADD8'; // Go Light Blue
        case 'md': return '#083fa1'; // Markdown Dark Blue
        default: return '#757575'; // 默认灰 Action
    }
};

interface TreeNode {
    name: string;
    path: string;
    isDirectory: boolean;
    children: TreeNode[];
    file?: FileInfo;
    allFileIds: number[];
}

function buildTree(files: FileInfo[]): TreeNode {
    const root: TreeNode = { name: 'root', path: '', isDirectory: true, children: [], allFileIds: [] };

    files.forEach(f => {
        const parts = f.relative_path.split('/');
        let curr = root;
        root.allFileIds.push(f.id);

        for (let i = 0; i < parts.length; i++) {
            const part = parts[i];
            const currentPath = parts.slice(0, i + 1).join('/');

            if (i === parts.length - 1) {
                curr.children.push({
                    name: part,
                    path: currentPath,
                    isDirectory: false,
                    children: [],
                    file: f,
                    allFileIds: [f.id]
                });
            } else {
                let child = curr.children.find(c => c.name === part && c.isDirectory);
                if (!child) {
                    child = { name: part, path: currentPath, isDirectory: true, children: [], allFileIds: [] };
                    curr.children.push(child);
                }
                child.allFileIds.push(f.id);
                curr = child;
            }
        }
    });

    const sortTree = (node: TreeNode) => {
        node.children.sort((a, b) => {
            if (a.isDirectory && !b.isDirectory) return -1;
            if (!a.isDirectory && b.isDirectory) return 1;
            return a.name.localeCompare(b.name);
        });
        node.children.forEach(sortTree);
    };
    sortTree(root);

    return root;
}

const TreeItem = ({ node, selectedIds, onToggleSelect, defaultExpanded = false }: any) => {
    const [expanded, setExpanded] = useState(defaultExpanded);

    const isSelected = node.allFileIds.length > 0 && node.allFileIds.every((id: number) => selectedIds.has(id));
    const isIndeterminate = !isSelected && node.allFileIds.some((id: number) => selectedIds.has(id));

    const handleToggle = () => {
        onToggleSelect(node, !isSelected);
    };

    return (
        <React.Fragment>
            <ListItem disablePadding sx={{ pl: node.path.split('/').length - 1 }}>
                <ListItemIcon sx={{ minWidth: '32px !important' }}> {/* 维持箭头占位宽幅 */}
                    {node.isDirectory ? (
                        <IconButton size="small" onClick={() => setExpanded(!expanded)} sx={{ p: 0 }}>
                            {expanded ? <ExpandMoreIcon fontSize="small" /> : <ChevronRightIcon fontSize="small" />}
                        </IconButton>
                    ) : (
                        <div style={{ width: 24 }} />
                    )}
                </ListItemIcon>
                <ListItemIcon sx={{ minWidth: '18px !important', ml: 1 }}> {/* 复选框的坑，通过 ml 让它离开箭头，宽度缩为28 */}
                    <Checkbox
                        edge="start"
                        checked={isSelected}
                        indeterminate={isIndeterminate}
                        disableRipple
                        size="small"
                        onChange={handleToggle}
                        sx={{ p: 0 }}
                    />
                </ListItemIcon>
                <ListItemIcon sx={{ minWidth: '28px !important' }}> {/* 您指出的绝对完美数值：18px，用!important强杀MUI覆盖 */}
                    {node.isDirectory ? (
                        <FolderOutlinedIcon fontSize="small" sx={{ color: '#FFCA28' }} />
                    ) : (
                        <InsertDriveFileOutlinedIcon
                            fontSize="small"
                            sx={{ color: getFileIconColor(node.file?.extension) }}
                        />
                    )}
                </ListItemIcon>
                <ListItemText
                    primary={node.name}
                    primaryTypographyProps={{ variant: 'body2', noWrap: true, sx: { fontSize: '0.88rem', ml: 0 } }} // 取消多余间隙
                />
            </ListItem>
            {node.isDirectory && (
                <Collapse in={expanded} timeout="auto" unmountOnExit={false}>
                    <List component="div" disablePadding>
                        {node.children.map((child: any) => (
                            <TreeItem
                                key={child.path}
                                node={child}
                                selectedIds={selectedIds}
                                onToggleSelect={onToggleSelect}
                            />
                        ))}
                    </List>
                </Collapse>
            )}
        </React.Fragment>
    );
};

export const FileTree = ({ files, selectedIds, onSelectionChange }: any) => {
    const rootNode = useMemo(() => buildTree(files), [files]);

    const handleToggleSelect = (node: TreeNode, willSelect: boolean) => {
        const newSelected = new Set(selectedIds);
        node.allFileIds.forEach((id: number) => {
            if (willSelect) {
                newSelected.add(id);
            } else {
                newSelected.delete(id);
            }
        });
        onSelectionChange(newSelected);
    };

    if (files.length === 0) return <div style={{ padding: 16, color: '#888', fontSize: 13 }}>暂无扫描结果，请先执行扫描...</div>;

    return (
        <List sx={{ width: '100%', bgcolor: 'background.paper', p: 0 }}>
            {rootNode.children.map((child: any) => (
                <TreeItem
                    key={child.path}
                    node={child}
                    selectedIds={selectedIds}
                    onToggleSelect={handleToggleSelect}
                    defaultExpanded={true}
                />
            ))}
        </List>
    );
};
