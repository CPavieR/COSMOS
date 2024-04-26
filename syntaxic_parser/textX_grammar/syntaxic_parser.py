import textx
import json


def is_valid_sql(query):
    # Get grammar file
    sql_meta = textx.metamodel_from_file("syntaxic_parser/textX_grammar/grammar_file.tx", ignore_case=True)
    try:
        # Analyse SQL query and get textX object
        model = sql_meta.model_from_str(query)

        # Call the handle_select_statement function with the textX object and get the corresponding dictionary
        result = handle_select_statement(model)

        # Convert the dictionary to a Json string
        json_result = json.dumps(result, indent=4)

        return json_result

    except textx.exceptions.TextXSyntaxError as e:
        # If the syntax is incorrect, return the error and its location

        error = f"Syntax Error line {e.line}, row {e.col}: {e.message} at : {query[e.col - 1:]}"

        return error


# If the syntax is correct, this function is called to create a dictionary with the elements of the query
def handle_select_statement(model):
    # Create the dict structure in which the elements of the query will be stored
    result = {
        "table_name": [],
        "columns": [],
        "where_clause": None
    }

    # TABLES
    if model.relations:
        # For every table, add it to the list of tables
        for relation in model.relations.relation:
            table_name = {
                "table_name": relation.relationName.upper(),
                "use_name_table": ""
            }

            # If the table is renamed
            if relation.alias:
                table_name["use_name_table"] = relation.alias
            else:
                table_name["use_name_table"] = table_name["table_name"]

            result["table_name"].append(table_name)

    # ATTRIBUTES
    if model.attributes:
        # If the attribute is a "*"
        if model.attributes.attribute == ['*']:
            columns = {
                "use_name_table": "",
                "attribute_name": "*",
                "use_name_attribute": ""
            }

            # If the table is specified
            if model.attributes.table:
                columns["use_name_table"] = model.attributes.table.upper()

            # If the '*' attribute is renamed with AS
            if model.attributes.alias:
                columns["use_name_attribute"] = model.attributes.alias
            else:
                columns["use_name_attribute"] = "*"

            result["columns"].append(columns)

        else:
            # If there is a list of attributes
            for attribute in model.attributes.attribute:

                columns = {
                    "use_name_table": "",
                    "attribute_name": "",
                    "use_name_attribute": ""
                }

                # If the attribute is an aggregate function
                if attribute.aggregate:
                    columns["attribute_name"] = attribute.aggregate.aggregateName + ',' + attribute.aggregate.attributeName.upper()

                    # If the table is specified
                    if attribute.aggregate.table:
                        columns["use_name_table"] = attribute.aggregate.table

                    # If the attribute is renamed with AS
                    if attribute.alias:
                        columns["use_name_attribute"] = attribute.alias
                    else:
                        columns["use_name_attribute"] = attribute.aggregate.aggregateName + '(' + attribute.aggregate.attributeName.upper() + ')'

                # If the attribute is a regular attribute
                else:
                    columns["attribute_name"] = attribute.attributeName.upper()

                    # If the table is specified
                    if attribute.table:
                        columns["use_name_table"] = attribute.table

                    # If the attribute is renamed with AS
                    if attribute.alias:
                        columns["use_name_attribute"] = attribute.alias
                    else:
                        columns["use_name_attribute"] = columns["attribute_name"]

                result["columns"].append(columns)

                # if attribute.distinctOption == "DISTINCT":
                #    result["conditions"] = "distinct" + columns["use_name_attribute"]

    # CONDITIONS
    if model.whereClause:
        # Call the handle_conditions function
        result["where_clause"] = handle_conditions(model.whereClause.conditions)

    # Return dictionary structure
    return result


def handle_conditions(condition_list_path):
    conditions = {
        "conditions": [],
        "linkers": []
    }

    # The first condition in a ConditionList, it has no linker attached to it
    if condition_list_path.in_condition:
        conditions["conditions"].append(handle_in_condition(condition_list_path.in_condition))
    elif condition_list_path.condition:
        cond = condition_list_path.condition

        # If it is a condition list between brackets, call handle_conditions again
        if cond.prioritised_conditions:
            conditions["conditions"].append(handle_conditions(cond.prioritised_conditions))
        else:
            conditions["conditions"].append(handle_single_condition(cond))

    # Linkers list is empty for single condition

    # Following conditions
    if condition_list_path.linked_condition:
        for cond in condition_list_path.linked_condition:
            # Add the linker to the list
            conditions["linkers"].append(cond.linker)

            if cond.in_condition:
                conditions["conditions"].append(handle_in_condition(cond.in_condition))
            # If it is a condition list between brackets, call handle_conditions again
            elif cond.condition.prioritised_conditions:
                conditions["conditions"].append(handle_conditions(cond.condition.prioritised_conditions))
            # Else, call single_condition
            else:
                conditions["conditions"].append(handle_single_condition(cond.condition))
    return conditions


def handle_single_condition(single_condition_path):
    cond = single_condition_path
    struct_condition = {
        "left": None,
        "op": str(cond.op),
        "right": None,
    }

    # LEFT
    # If the left side is a subquery, call handle_select_statement with the textX object that is the subquery
    if cond.leftSubquery:
        struct_condition["left"] = handle_select_statement(cond.leftSubquery.subquery)
    else:
        # If it's a static value
        if cond.left:
            struct_condition["left"] = cond.left

        else:
            # If it's an attribute and the table is specified
            if cond.leftAttribute.table:
                struct_condition["left"] = cond.leftAttribute.table + '.' + cond.leftAttribute.attr
            else:
                struct_condition["left"] = cond.leftAttribute.attr

    # RIGHT
    # If the right side is a subquery, call handle_select_statement with the textX object that is the subquery
    if cond.rightSubquery:
        struct_condition["right"] = handle_select_statement(cond.rightSubquery.subquery)

    else:
        # If it's a static value
        if cond.right:
            struct_condition["right"] = cond.right

        else:
            # If it's an attribute and the table is specified
            if cond.rightAttribute.table:
                struct_condition["right"] = cond.rightAttribute.table + '.' + cond.rightAttribute.attr
            else:
                struct_condition["right"] = cond.rightAttribute.attr
    return struct_condition


def handle_in_condition(in_condition_path):
    cond = in_condition_path
    struct_condition = {
        "left": cond.leftAttribute.attr,
        "op": 'IN',
        "right": [],
    }

    if cond.leftAttribute.table:
        struct_condition["left"] = cond.leftAttribute.table + '.' + cond.leftAttribute.attr

    if cond.notOption:
        struct_condition["op"] = 'NOT ' + struct_condition["op"]

    for right in cond.right:
        struct_condition["right"].append(right)
    return struct_condition
