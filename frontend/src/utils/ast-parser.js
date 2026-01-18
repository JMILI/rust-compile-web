// AST 解析器 - 将 Rust 的调试输出转换为 JSON 树状结构

/**
 * 解析 AST 文本为结构化数据
 * @param {string} astText - AST 文本内容
 * @returns {Object} 解析后的树状结构
 */
export function parseAstToTree(astText) {
    try {
        // 首先尝试直接解析为 JSON（如果后端能输出 JSON 格式）
        if (astText.trim().startsWith('{') || astText.trim().startsWith('[')) {
            try {
                return JSON.parse(astText);
            } catch (e) {
                // 如果不是有效的 JSON，继续使用文本解析
            }
        }

        // 解析文本格式的 AST
        return parseTextAst(astText);
    } catch (error) {
        console.error('Failed to parse AST:', error);
        return {
            id: 'error',
            label: '解析失败',
            children: [{
                id: 'error-msg',
                label: error.message
            }]
        };
    }
}

/**
 * 解析文本格式的 AST
 */
function parseTextAst(text) {
    const lines = text.split('\n');
    let root = null;
    const stack = [];
    let currentNode = null;

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i].trim();

        // 跳过空行
        if (!line) continue;

        // 检测行缩进（用于判断层级）
        const indent = lines[i].match(/^(\s*)/)[1].length;
        const level = Math.floor(indent / 2); // 假设每2个空格为一级

        // 解析节点类型和内容
        const node = parseAstLine(line);

        if (!root) {
            // 第一个非空行作为根节点
            root = node;
            currentNode = root;
            stack.push({ node: root, level });
        } else {
            // 调整栈以匹配当前层级
            while (stack.length > 0 && stack[stack.length - 1].level >= level) {
                stack.pop();
            }

            // 获取父节点
            const parent = stack.length > 0 ? stack[stack.length - 1].node : root;

            // 添加到父节点
            if (!parent.children) {
                parent.children = [];
            }
            parent.children.push(node);

            // 如果节点可能有子节点，压入栈
            if (line.includes('{') || line.includes('[')) {
                stack.push({ node, level });
            }
        }
    }

    return root || {
        id: 'empty',
        label: '空的 AST',
        children: []
    };
}

/**
 * 解析单行 AST 文本
 */
function parseAstLine(line) {
    // 提取节点类型和属性
    let nodeType = 'Unknown';
    let nodeContent = line;
    let nodeId = generateId();

    // 尝试提取节点类型
    const typeMatch = line.match(/^([A-Z][a-zA-Z]*)/);
    if (typeMatch) {
        nodeType = typeMatch[1];
    }

    // 尝试提取标识符
    const identMatch = line.match(/ident:\s*([^,}\s]+)/);
    const ident = identMatch ? identMatch[1] : null;

    // 尝试提取字面量值
    const litMatch = line.match(/symbol:\s*"([^"]+)"/);
    const literal = litMatch ? litMatch[1] : null;

    // 构建节点标签
    let label = nodeType;
    if (ident) {
        label += `: ${ident}`;
    } else if (literal) {
        label += `: "${literal}"`;
    }

    // 如果行很短，直接显示整行
    if (line.length < 50) {
        label = line.replace(/\s+/g, ' ').trim();
    }

    return {
        id: nodeId,
        label,
        type: nodeType.toLowerCase(),
        fullText: line,
        children: []
    };
}

/**
 * 生成唯一 ID
 */
function generateId() {
    return 'node_' + Math.random().toString(36).substr(2, 9);
}

/**
 * 简化 AST 树（可选，用于大型 AST）
 */
export function simplifyAstTree(tree, maxDepth = 5, maxChildren = 10) {
    function simplify(node, depth = 0) {
        if (!node) return null;

        const simplified = {
            id: node.id,
            label: node.label,
            type: node.type,
            depth
        };

        // 限制深度和子节点数量
        if (depth < maxDepth && node.children && node.children.length > 0) {
            simplified.children = node.children
                .slice(0, maxChildren)
                .map(child => simplify(child, depth + 1))
                .filter(Boolean);

            // 如果有被截断的子节点，添加提示
            if (node.children.length > maxChildren) {
                simplified.children.push({
                    id: node.id + '_more',
                    label: `... 还有 ${node.children.length - maxChildren} 个子节点`,
                    type: 'info',
                    depth: depth + 1
                });
            }
        }

        return simplified;
    }

    return simplify(tree);
}

/**
 * 将 AST 树转换为 G6 图数据格式
 */
export function convertToGraphData(tree) {
    if (!tree) return { nodes: [], edges: [] };

    const nodes = [];
    const edges = [];
    let nodeId = 0;

    function traverse(node, parentId = null, depth = 0) {
        const id = node.id || `node_${nodeId++}`;

        // 根据节点类型设置不同的样式
        let style = {
            fill: '#1890ff',
            stroke: '#1890ff'
        };

        if (node.type) {
            switch (node.type.toLowerCase()) {
                case 'crate':
                    style.fill = '#722ed1';
                    style.stroke = '#722ed1';
                    break;
                case 'item':
                case 'fn':
                    style.fill = '#13c2c2';
                    style.stroke = '#13c2c2';
                    break;
                case 'block':
                    style.fill = '#52c41a';
                    style.stroke = '#52c41a';
                    break;
                case 'stmt':
                case 'expr':
                    style.fill = '#fa8c16';
                    style.stroke = '#fa8c16';
                    break;
                case 'lit':
                    style.fill = '#f5222d';
                    style.stroke = '#f5222d';
                    break;
                default:
                    style.fill = '#1890ff';
                    style.stroke = '#1890ff';
            }
        }

        nodes.push({
            id,
            label: node.label || '节点',
            style,
            depth,
            type: 'circle',
            size: 32 - depth * 4,
            labelCfg: {
                style: {
                    fontSize: 12 - depth,
                    fill: '#fff'
                }
            }
        });

        // 添加边
        if (parentId) {
            edges.push({
                source: parentId,
                target: id,
                style: {
                    stroke: '#8c8c8c',
                    lineWidth: 1,
                    opacity: 0.6
                }
            });
        }

        // 递归处理子节点
        if (node.children && node.children.length > 0) {
            node.children.forEach(child => {
                traverse(child, id, depth + 1);
            });
        }

        return id;
    }

    traverse(tree);

    return { nodes, edges };
}

/**
 * 从 AST 文本中提取关键信息
 */
export function extractAstInfo(astText) {
    const info = {
        functions: [],
        variables: [],
        literals: [],
        structures: []
    };

    // 提取函数定义
    const fnMatches = astText.match(/Fn\s*{[\s\S]*?ident:\s*([^,\s}]+)/g);
    if (fnMatches) {
        info.functions = fnMatches.map(match => {
            const identMatch = match.match(/ident:\s*([^,\s}]+)/);
            return identMatch ? identMatch[1] : null;
        }).filter(Boolean);
    }

    // 提取变量声明
    const varMatches = astText.match(/Local\s*{[\s\S]*?pat:\s*Pat\s*{[\s\S]*?Ident[\s\S]*?([^,\s}]+)[^}]*?}/g);
    if (varMatches) {
        info.variables = varMatches.map(match => {
            const varMatch = match.match(/Ident[^,]*,\s*([^,\s}]+)/);
            return varMatch ? varMatch[1] : null;
        }).filter(Boolean);
    }

    // 提取字面量
    const litMatches = astText.match(/symbol:\s*"([^"]+)"/g);
    if (litMatches) {
        info.literals = litMatches.map(match => {
            const litMatch = match.match(/"([^"]+)"/);
            return litMatch ? litMatch[1] : null;
        }).filter(Boolean);
    }

    // 提取结构体
    const structMatches = astText.match(/(Struct|Enum|Trait|Impl)\s*{[\s\S]*?ident:\s*([^,\s}]+)/g);
    if (structMatches) {
        info.structures = structMatches.map(match => {
            const structMatch = match.match(/(Struct|Enum|Trait|Impl)[^}]*ident:\s*([^,\s}]+)/);
            return structMatch ? `${structMatch[1]}: ${structMatch[2]}` : null;
        }).filter(Boolean);
    }

    return info;
}

/**
 * 获取 AST 统计信息
 */
export function getAstStats(astText) {
    const lines = astText.split('\n');
    const nonEmptyLines = lines.filter(line => line.trim());

    return {
        totalLines: lines.length,
        nonEmptyLines: nonEmptyLines.length,
        indentLevels: getMaxIndentLevel(astText),
        nodeCount: countNodes(astText),
        estimatedSize: astText.length
    };
}

function getMaxIndentLevel(text) {
    const lines = text.split('\n');
    let maxIndent = 0;

    lines.forEach(line => {
        const indent = line.match(/^(\s*)/)[1].length;
        const level = Math.floor(indent / 2);
        if (level > maxIndent) {
            maxIndent = level;
        }
    });

    return maxIndent;
}

function countNodes(text) {
    // 简单计数：每行包含 '{' 或 '[' 并且有节点类型
    const nodePattern = /[A-Z][a-zA-Z]*\s*[{\[]/g;
    const matches = text.match(nodePattern);
    return matches ? matches.length : 0;
}